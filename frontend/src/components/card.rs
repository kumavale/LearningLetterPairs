use gloo_net::http::Request;
use serde::Serialize;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlFormElement, FormData};
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
                {&pair}
                <div class="dropdown">
                    <button class="btn dropdown-toggle" type="button" id="dropdownMenu"
                        data-bs-toggle="dropdown" aria-haspopup="true" aria-expanded="false">
                        {"\u{FE19}"}
                    </button>
                    <div class="dropdown-menu" aria-labelledby="dropdownMenu">
                        <button class="dropdown-item" type="button">{"Modify"}</button>
                        <form onsubmit={delete}>
                            <input type="hidden" name="pair" value={pair.clone()} />
                            <button class="dropdown-item fw-bold text-danger" type="submit">{"Delete"}</button>
                        </form>
                    </div>
                </div>
            </div>
            <div class="card-body" style="text-align: center;">
                <p class="card-text" style="text-align: left;">{object}</p>
                <img class="card-img-bottom" src={props.img.clone()} />
            </div>
        </div>
    }
}

fn delete(e: SubmitEvent) {
    let target: Option<EventTarget> = e.target();
    let form: HtmlFormElement = target.and_then(|t| t.dyn_into::<HtmlFormElement>().ok()).unwrap();
    let form_data = FormData::new_with_form(&form).unwrap();

    // TODO: 型はクラサバ共有
    #[derive(Serialize)]
    struct LetterPair { pair: String, }
    let pair = LetterPair { pair: form_data.get("pair").as_string().unwrap(), };

    wasm_bindgen_futures::spawn_local(async move {
        if let Err(_e) = Request::delete("http://localhost:3000/pairs")
            .json(&pair)
            .unwrap()
            .send()
            .await
        {
            // TODO: DELETE失敗
        }
    });

    // TODO: レスポンスからカードを削除

    e.prevent_default();
}
