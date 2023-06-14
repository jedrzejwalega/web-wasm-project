use std::{net::{IpAddr, Ipv6Addr, SocketAddr}, str::FromStr};
use clap::Parser;
use axum::{Router, routing::get, response::IntoResponse};
use hyper::server::{Builder, conn::AddrIncoming};

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

    let server_to_serve:Builder<AddrIncoming> = match axum::Server::try_bind(&socket_address){
                                                        Ok(server_builder) => server_builder,
                                                        Err(_)=> panic!("Failed to set up server at socket address: {}", socket_address ) 
                                                        };
    println!("listening on http://{}", socket_address);
    server_to_serve.serve(app.into_make_service()).await.expect("Unable to start the server!");
}

async fn root() -> impl IntoResponse{
    "Hello World!"
}
