use std::net::SocketAddr;
use std::sync::Arc;
use quinn::Endpoint;
use tokio::net::TcpListener;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;
use crate::architecture::ServerConfiguration;

pub struct Server {
    config: ServerConfiguration,
    endpoint: Option<Endpoint>, // Authorize None
    ports: Mutex<Vec<u16>> // Avoid mut
}

impl Server {
    pub fn new(config: ServerConfiguration) -> Arc<Self> {
        Arc::new(
            Server {
                config,
                endpoint: None,
                ports: Mutex::new(Vec::new())
            }
        )
    }
}

pub struct AccessServer {
    address: SocketAddr,
    sender: Sender<Option<ChannelMess>>,
    receiver: Option<Receiver<Option<ChannelMess>>>,
    listener: Option<Arc<TcpListener>>,
    drop: Arc<Mutex<bool>>
}