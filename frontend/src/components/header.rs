use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <>
            <heaer>
                <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
                    <a class="navbar-brand m-1 ms-4 h1" href="#">{"Learning Letter Pairs"}</a>
                </nav>
            </heaer>
        </>
    }
}
