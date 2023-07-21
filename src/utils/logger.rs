use quinn::{RecvStream, SendStream};
use tokio::net::tcp::OwnedReadHalf;

pub fn log_tunnel(tcp_read: OwnedReadHalf, quic_send: SendStream) -> () {
    println!("======== FP VPN ========\n");
    println!("Tunnel created {} <--> {}\n", tcp_read.peer_addr().unwrap(), quic_send.id().index());
    println!("========================\n");
}