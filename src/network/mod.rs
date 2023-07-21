use lazy_static::lazy_static;
use byte_pool::BytePool;

pub mod tunnel;

pub (crate) enum ReadResult {
    End,
    Success,
}

lazy_static! {
    static ref BUFFER: BytePool::<Vec<u8>> = BytePool::<Vec<u8>>::new();
}