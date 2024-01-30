use std::net::SocketAddr;

pub mod client;
pub mod server;

pub struct ServerConfiguration {
    pub address: String,
    pub key: String,
    pub certificate: String,

    // servers which will relay the traffic
    pub upstream: Vec<SocketAddr>,

    // XX.XX.XX.XX:XXXX
    pub dashboard: String,

    // user:pass
    pub dashboard_auth: String
}

pub struct ClientConfiguration {}