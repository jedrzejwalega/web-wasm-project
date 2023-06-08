use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "server", about = "wasm project server")]
struct Server {
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    addr: String,
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16
}

#[tokio::main]
async fn main() {
    let serv = Server::parse();
}
