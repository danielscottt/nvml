extern crate libc;

mod pool;
pub use self::pool::{PoolErr, PoolResult, PmemPool};

mod obj;
pub use obj::PmemObj;

#[test]
fn it_works() {}
