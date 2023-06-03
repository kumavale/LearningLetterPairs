use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <>
            <heaer>
                <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
                    <a class="navbar-brand m-1 ms-4 me-4 h1" href="/">{"Learning Letter Pairs"}</a>
                    <button class="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarNavAltMarkup" aria-controls="navbarNavAltMarkup" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                    </button>
                    <div class="collapse navbar-collapse" id="navbarNavAltMarkup">
                        <div class="navbar-nav">
                        <a class="nav-item nav-link" href="/list">{"List"}</a>
                        <a class="nav-item nav-link disabled" href="/add">{"Add"}</a>
                        <a class="nav-item nav-link disabled" href="/quiz">{"Quiz"}</a>
                        </div>
                    </div>
                </nav>
            </heaer>
        </>
    }
}
