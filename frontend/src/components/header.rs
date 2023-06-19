use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
                <a class="navbar-brand m-1 ms-4 me-4 h1" href="/">{"Learning Letter Pairs"}</a>
                <div class="ms-auto">
                    <a class="btn btn-outline-light me-2" href="/login">{"Login"}</a>
                </div>
            </nav>
        </header>
    }
}
