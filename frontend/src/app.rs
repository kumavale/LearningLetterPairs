use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::header::Header;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/list")]
    List,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Top)]
pub fn top() -> Html {
    html! {
        <>
        <div class="contents">
            <h1>{"Welcome to Learning Letter Pairs !"}</h1>
            <p>{"Hi, anonymous !"}</p>
            <p>
                {"This site provides the best content for learning letter pairs."}<br />
                {"You can specify not only character combinations, but also images."}<br />
            </p>
        </div>
        </>
    }
}

#[function_component(List)]
pub fn list() -> Html {
    html! {
        <>
            <h1>{"List of letter pairs"}</h1>
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Top /> },
        Route::List => html! { <List /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
        <Header />
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
        </>
    }
}
