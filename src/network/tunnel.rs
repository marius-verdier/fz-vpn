extern crate tokio;
use anyhow::Result;
use quinn::{RecvStream, SendStream};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::utils::logger;
use crate::network::ReadResult;
use crate::network::BUFFER;

const SIZE: usize = 8192;

pub struct Tunnel {

}

impl Tunnel {

    pub fn new() -> Self() {
        println!("New tunnel created");
        Tunnel {}
    }

    fn start(&self, tcp: (OwnedReadHalf, OwnedWriteHalf), quic: (RecvStream, SendStream)) -> () {
        let (mut tcp_read, mut tcp_write) = tcp;
        let (mut quic_rec, mut quic_send) = quic;

        logger::log_tunnel(tcp_read, quic_send);

        tokio::spawn(async move {
            loop {
                let result = self.tcp_quic(&mut tcp_read, &mut quic_send).await;
                match result {
                    Ok(ReadResult::Success) => (),
                    Ok(ReadResult::End) => break,
                    Err(e) => {
                        println!("Error during TCP -> QUIC: {}", e);
                        break;
                    },
                }
            }
        });

        tokio::spawn(async move {
            loop {
                let result = self.quic_tcp(&mut quic_rec, &mut tcp_write).await;
                match result {
                    Ok(ReadResult::Success) => (),
                    Ok(ReadResult::End) => break,
                    Err(e) => {
                        println!("Error during QUIC -> TCP: {}", e);
                        break;
                    },
                }
            }
        });
    }

    async fn tcp_quic(&self, tcp_read : &mut OwnedReadHalf, quic_send : &mut SendStream) -> Result<ReadResult> {
        let mut buf = BUFFER.alloc_and_fill(SIZE);
        let len = tcp_read.read(&mut buf[..]).await?;
        if len > 0 {
            quic_send.write_all(&buf[..len]).await?;
            return Ok(ReadResult::Success);
        } else {
            return Ok(ReadResult::End);
        }
    }

    async fn quic_tcp(&self, quic_rec : &mut RecvStream, tcp_write : &mut OwnedWriteHalf) -> Result<ReadResult> {
        let mut buf = BUFFER.alloc_and_fill(SIZE);
        let res = quic_rec.read(&mut buf[..]).await?;
        if let Some(len) = res {
            tcp_write.write_all(&buf[..len]).await?;
            return Ok(ReadResult::Success);
        } else {
            return Ok(ReadResult::End);
        }
    }

}