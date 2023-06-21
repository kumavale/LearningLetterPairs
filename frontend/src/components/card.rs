use yew::prelude::*;
use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub pair: String,
    pub object: String,
    pub img: String,
}

#[function_component(Card)]
pub fn card(props: &Props) -> Html {
    let pair = &props.pair;
    let object = &props.object;

    html! {
        <div class="card bg-light mb-3 card-pair">
            <div class="card-header d-flex justify-content-between align-items-center" style="padding: 0 0 0 16px;">
                {pair}
                <div class="dropdown">
                    <button class="btn dropdown-toggle" type="button" id="dropdownMenu"
                        data-bs-toggle="dropdown" aria-haspopup="true" aria-expanded="false">
                        {"\u{FE19}"}
                    </button>
                    <div class="dropdown-menu" aria-labelledby="dropdownMenu">
                        <button class="dropdown-item" type="button">{"Modify"}</button>
                        <button class="dropdown-item fw-bold text-danger" type="button">{"Delete"}</button>
                    </div>
                </div>
            </div>
            <div class="card-body">
                <p class="card-text">{object}</p>
                <img class="card-img-bottom" src={props.img.clone()} />
            </div>
        </div>
    }
}
