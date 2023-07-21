pub mod network;
pub mod utils;

fn main() {
    println!("Welcome to the most wonderful VPN in the world! If you want foxy's password, don't hesitate");
    network::tunnel::Tunnel::new();
}