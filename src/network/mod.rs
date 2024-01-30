use lazy_static::lazy_static;
use byte_pool::BytePool;
use quinn::{RecvStream, SendStream};

pub mod tunnel;
pub mod bridge;
mod message;

pub (crate) enum ReadResult {
    End,
    Success,
}

lazy_static! {
    static ref BUFFER: BytePool::<Vec<u8>> = BytePool::<Vec<u8>>::new();
}

#[derive(Debug)]
pub enum TunnelType {
    In, // Connection, access server and stream controller
    Out, // Connection, socket address
    InAndOut, // Connection, socket address, access server and stream controller
}

pub struct StreamControl {
    pub q_send: SendStream,
    pub q_recv: RecvStream,
}