use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <heaer>
            <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
                <a class="navbar-brand m-1 ms-4 me-4 h1 flex-grow-grow-1" href="/">{"Learning Letter Pairs"}</a>
                <div class="justify-content-end">
                    <a class="btn btn-outline-light me-2" href="/login">{"Login"}</a>
                </div>
            </nav>
        </heaer>
    }
}
