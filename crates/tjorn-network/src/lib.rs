pub mod dns;
pub mod http;
pub mod quic;
pub mod tcp;
pub mod udp;
pub mod vpn;

pub use http::HttpServer;
pub use tcp::TcpStack;
pub use udp::UdpStack;

pub fn init() {
    println!("Initializing {}", "tjorn-network");
}
