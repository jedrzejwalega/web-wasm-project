use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route{
    #[at("/")]
    Home
}

fn switch( available_routes:Route) -> Html {
    match available_routes{
        Route::Home => html!{ <h1>{ "Hello Frontend!"}</h1>}
    }
}

#[function_component(App)]
fn app() -> Html {
    html!{
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}


fn main() {
    let log_config = wasm_logger::Config::new(log::Level::Trace);
    wasm_logger::init(log_config);
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
