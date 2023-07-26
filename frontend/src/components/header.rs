use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub username: Option<String>,
}

#[function_component(Header)]
pub fn header(props: &Props) -> Html {
    let foo = if props.username.is_some() {
        html! {
            <a class="btn btn-outline-light me-2" href="/logout">{"Logout"}</a>
        }
    } else {
        html! {
            <a class="btn btn-outline-light me-2" href="/login">{"Login"}</a>
        }
    };

    html! {
        <header>
            <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
                <a class="navbar-brand m-1 ms-4 me-4 h1" href="/">{"Learning Letter Pairs"}</a>
                <div class="ms-auto d-flex flex-wrap">
                    { foo }
                </div>
            </nav>
            //<nav class="navbar navbar-expand-lg navbar-dark bg-dark">
            //    <div class="d-flex flex-wrap align-items-center justify-content-center justify-content-lg-start">
            //        <a class="navbar-brand m-1 ms-4 me-4 h1" href="/">{"Learning Letter Pairs"}</a>

            //        <div class="nav col-12 col-lg-auto me-lg-auto mb-2 justify-content-center mb-md-0"></div>

            //        //<form class="col-12 col-lg-auto mb-3 mb-lg-0 me-lg-3">
            //        //    <input type="search" class="form-control" placeholder="Search..." aria-label="Search" />
            //        //</form>

            //        <div class="dropdown flex-shrink-1 bd-highlight me-lg-3 justify-content-cnter my-md-0 col-12 col-lg-auto">
            //            <a href="#" class="d-block link-dark text-decoration-none dropdown-toggle" id="dropdownUser1" data-bs-toggle="dropdown" aria-expanded="false">
            //                <img src="https://github.com/kumavale.png" width="32" height="32" class="rounded-circle" />
            //            </a>
            //            <ul class="dropdown-menu text-small" aria-labelledby="dropdownUser1" style="">
            //                <li><a class="dropdown-item" href="#">{"Settings"}</a></li>
            //                <li><hr class="dropdown-divider" /></li>
            //                <li><a class="dropdown-item" href="#">{"Sign out"}</a></li>
            //            </ul>
            //        </div>
            //    </div>
            //</nav>
        </header>
    }
}
