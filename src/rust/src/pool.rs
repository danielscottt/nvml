extern crate libc;

use std::ptr;

use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;

use libc::{c_int, c_void, size_t};

use obj::PmemObj;

#[link(name="pmem")]
extern "C" {
    fn pmem_is_pmem(addr: *const c_void, len: size_t) -> c_int;
    fn pmem_persist(addr: *const c_void, len: size_t);
    fn pmem_msync(addr: *const c_void, len: size_t) -> c_int;
    fn pmem_map(fd: c_int) -> *mut c_void;
    fn pmem_unmap(addr: *const c_void, len: size_t) -> c_int;
}

#[derive(Debug)]
pub enum PoolErr {
    OpenFailed,
    MmapFailed,
}

pub type PoolResult<T> = Result<T, PoolErr>;

pub struct PmemPool {
    size: size_t,
    is_pmem: c_int,
    addr: *mut c_void,
}

impl PmemPool {
    pub fn open(path: &str, size: u64) -> PoolResult<Self> {
        let file = match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path) {
            Ok(f) => f,
            Err(_) => return Err(PoolErr::OpenFailed),
        };
        match file.set_len(size) {
            Ok(_) => (),
            Err(_) => return Err(PoolErr::OpenFailed),
        }

        let fd = file.as_raw_fd();

        let addr = unsafe { pmem_map(fd as c_int) };
        if addr.is_null() {
            return Err(PoolErr::MmapFailed);
        }

        let c_size = size as size_t;
        let is_pmem = unsafe { pmem_is_pmem(addr, c_size) };

        Ok(PmemPool {
            size: c_size,
            is_pmem: is_pmem,
            addr: addr,
        })
    }

    pub fn sync(&self) {
        if self.is_pmem() {
            unsafe { pmem_persist(self.addr, self.size) };
        } else {
            unsafe { pmem_msync(self.addr, self.size) };
        }
    }

    pub fn is_pmem(&self) -> bool {
        if self.is_pmem == 1 {
            true
        } else {
            false
        }
    }

    pub fn close(&self) {
        unsafe { pmem_unmap(self.addr, self.size) };
    }

    pub fn set_root<T>(&self, d: T, size: usize) {
        let p = PmemObj::new(d, size);
        unsafe {
            let po_ptr = &mut *(self.addr as *mut PmemObj<T>);
            ptr::copy(&p, po_ptr, 1);
        };
    }

    pub fn get_root<T>(&self) -> &PmemObj<T> {
        unsafe { &(*(self.addr as *mut PmemObj<T>)) }
    }

    pub fn get_root_mut<T>(&self) -> &mut PmemObj<T> {
        unsafe { &mut (*(self.addr as *mut PmemObj<T>)) }
    }
}
