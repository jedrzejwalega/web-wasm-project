use std::{net::{IpAddr, Ipv6Addr, SocketAddr}, str::FromStr};
use clap::Parser;
use axum::{Router, routing::get, response::IntoResponse};
use hyper::server::{Builder, conn::AddrIncoming};
use tower_http::trace::TraceLayer;
use axum::body::{boxed, Body};
use axum::http::{Response, StatusCode};
use tower::{ServiceBuilder, ServiceExt};
use tower_http::services::ServeDir;

#[derive(Parser, Debug)]
#[clap(name = "server", about = "wasm project server")]
struct Serv {
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    addr: String,
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,
    #[clap(short = 'l', long = "log",default_value = "debug")]
    log_level: String,
    #[clap(short = 's', long= "static-directory", default_value = "./dist")]
    static_dir: String
}

// development only function
// fn type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

#[tokio::main]
async fn main() {
    let server_parameters = Serv::parse();
    let logging_layer = ServiceBuilder::new()
                        .layer(TraceLayer::new_for_http());
    let app: Router = Router::new()
                                .route("/api/hello", get(root))
                                .fallback_service(get(|req| async move {
    match ServeDir::new(server_parameters.static_dir).oneshot(req).await {
        Ok(res) => res.map(boxed),
        Err(err) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(boxed(Body::from(format!("error: {err}"))))
                    .expect("error response"),
        }}))
        .layer(logging_layer);
    let localhost_address:IpAddr = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let server_address:IpAddr = IpAddr::from_str(server_parameters.addr.as_str())
                                                .unwrap_or(localhost_address);
    let socket_address:SocketAddr = SocketAddr::from((server_address, server_parameters.port));

    if std::env::var("RUST_LOG").is_err(){
        std::env::set_var("RUST_LOG", format!("{}, hyper=info, mio=info", server_parameters.log_level));
    }
    tracing_subscriber::fmt::init();
    let server_to_serve:Builder<AddrIncoming> = match axum::Server::try_bind(&socket_address){
                                                        Ok(server_builder) => server_builder,
                                                        Err(_)=> panic!("Failed to set up server at socket address: {}", socket_address ) 
                                                        };
    log::info!("listening on http://{}", socket_address);
    server_to_serve.serve(app.into_make_service()).await.expect("Unable to start the server!");
}

async fn root() -> impl IntoResponse{
    "Hello World!"
}