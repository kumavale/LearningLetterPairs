use crate::components::header::Header;
use crate::login::{Login, Register, Terms};
use crate::top::Top;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/terms")]
    Terms,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Top /> },
        Route::Login => html! { <Login /> },
        Route::Register => html! { <Register /> },
        Route::Terms=> html! { <Terms /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    let username = local_storage.get("username").unwrap();

    html! {
        <>
        <Header username={username} />
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
        </>
    }
}
