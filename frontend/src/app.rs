use yew::prelude::*;
use crate::components::header::Header;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Header />
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
