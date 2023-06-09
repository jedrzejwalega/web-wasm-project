use std::{net::{IpAddr, Ipv6Addr, SocketAddr}, str::FromStr};
use clap::Parser;
use axum::{Router, routing::get};

#[derive(Parser, Debug)]
#[clap(name = "server", about = "wasm project server")]
struct Serv {
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    addr: String,
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16
}

// development only function
// fn type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

#[tokio::main]
async fn main() {
    let server_parameters = Serv::parse();
    let app: Router = Router::new()
                                .route("/", get(root));
    let localhost_address:IpAddr = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let server_address:IpAddr = IpAddr::from_str(server_parameters.addr.as_str())
                                                .unwrap_or(localhost_address);
    let socket_address:SocketAddr = SocketAddr::from((server_address, server_parameters.port));

}

async fn root() -> &'static str {
    "Hello World!"
}
