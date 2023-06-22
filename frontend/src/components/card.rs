use gloo_net::http::Request;
use serde::Serialize;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlFormElement, HtmlInputElement, FormData};
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
        <div class="card bg-light mb-3 card-pair" id={format!("card-{}", &pair)}>
            <div class="card-header d-flex justify-content-between align-items-center" style="padding: 0 0 0 16px;">
                {&pair}
                <div class="dropdown">
                    <button class="btn dropdown-toggle" type="button" id="dropdownMenu"
                        data-bs-toggle="dropdown" aria-haspopup="true" aria-expanded="false">
                        {"\u{FE19}"}
                    </button>
                    <div class="dropdown-menu" aria-labelledby="dropdownMenu">
                        <form onsubmit={modify}>
                            <input type="hidden" name="pair" value={pair.clone()} />
                            <input type="hidden" name="object" value={object.clone()} />
                            <button class="dropdown-item" type="submit" data-bs-toggle="modal" data-bs-target="#modify-modal">{"Modify"}</button>
                        </form>
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

fn modify(e: SubmitEvent) {
    // `value`を書き換える
    let target: Option<EventTarget> = e.target();
    let form: HtmlFormElement = target.and_then(|t| t.dyn_into::<HtmlFormElement>().ok()).unwrap();
    let form_data = FormData::new_with_form(&form).unwrap();
    let pair = form_data.get("pair").as_string().unwrap();
    let object = form_data.get("object").as_string().unwrap();
    let document = web_sys::window().unwrap().document().unwrap();
    document.get_element_by_id("modifyInputPair")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .set_value(&pair);
    document.get_element_by_id("modifyInputObject")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .set_value(&object);

    e.prevent_default();
}

fn delete(e: SubmitEvent) {
    let target: Option<EventTarget> = e.target();
    let form: HtmlFormElement = target.and_then(|t| t.dyn_into::<HtmlFormElement>().ok()).unwrap();
    let form_data = FormData::new_with_form(&form).unwrap();

    // TODO: 型はクラサバ共有
    #[derive(Serialize)]
    struct LetterPair { pair: String, }
    let pair = form_data.get("pair").as_string().unwrap();
    let data = LetterPair { pair: pair.to_string() };

    wasm_bindgen_futures::spawn_local(async move {
        if let Err(_e) = Request::delete("http://localhost:3000/pairs")
            .json(&data)
            .unwrap()
            .send()
            .await
        {
            // TODO: DELETE失敗
        }
    });

    // カードを削除
    let document = web_sys::window().unwrap().document().unwrap();
    document.get_element_by_id(&format!("card-{}", &pair))
        .unwrap()
        .remove();

    e.prevent_default();
}
