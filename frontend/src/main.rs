use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route{
    #[at("/")]
    Home
}

fn main() {
    let log_config = wasm_logger::Config::new(log::Level::Trace);
    wasm_logger::init(log_config);
    console_error_panic_hook::set_once();
}
