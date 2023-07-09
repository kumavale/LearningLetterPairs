use gloo_net::http::Request;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{EventTarget, HtmlFormElement, HtmlInputElement, FormData};
use yew::prelude::*;
use yew::{Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub modify: bool,
}

#[function_component(AddButton)]
pub fn add_button() -> Html {
    html! {
        <button class="fixed-button" type="button" data-bs-toggle="modal" data-bs-target="#add-modal">{"\u{FF0B}"}</button>
    }
}

#[function_component(AddModal)]
pub fn add_modal(props: &Props) -> Html {
    let id_prefix = if props.modify { "modify" } else { "add" };
    let is_modify = props.modify;

    // モーダルウィンドウの最初のフォーカスを設定
    use_effect(move || {
        let oncustard = Callback::from(move |_: Event| {
            web_sys::window().unwrap()
                .document().unwrap()
                .get_element_by_id(if is_modify { "modifyInputObject" } else { "addInputPair" }).unwrap()
                .dyn_into::<HtmlInputElement>().unwrap()
                .focus().unwrap();
        });
        let listener =
            Closure::<dyn Fn(Event)>::wrap(
                Box::new(move |e: Event| { oncustard.emit(e) })
            );
        web_sys::window().unwrap()
            .document().unwrap()
            .get_element_by_id(if is_modify { "modify-modal" } else { "add-modal" }).unwrap()
            .add_event_listener_with_callback("shown.bs.modal", listener.as_ref().unchecked_ref()).unwrap();
        move || drop(Some(listener))
    });

    let onsubmit = Callback::from(move |e: SubmitEvent| {
        let target: Option<EventTarget> = e.target();
        let form: HtmlFormElement = target.and_then(|t| t.dyn_into::<HtmlFormElement>().ok()).unwrap();
        let form_data = FormData::new_with_form(&form).unwrap();

        wasm_bindgen_futures::spawn_local(async move {
            let request = if is_modify {
                Request::put(&format!("{}/pairs", crate::BACKEND_URL))
            } else {
                Request::post(&format!("{}/pairs", crate::BACKEND_URL))
            };
            if let Err(_e) = request
                .body(&form_data)
                .unwrap()
                .send()
                .await
            {
                // TODO: POST失敗
            }

            web_sys::window().unwrap().location().reload().ok();
        });

        // TODO: レスポンスからカードを生成して追加
        //html! {
        //    <Card pair={format!("{}{}", &pair.initial, &pair.next)} object={pair.object.clone()} img={pair.image.clone()} />
        //}

        e.prevent_default();
    });

    html! {
        <div class="modal fade" id={format!("{id_prefix}-modal")} tabindex="-1">
            <div class="modal-dialog modal-dialog-centered">
                <div class="modal-content">
                    <form onsubmit={onsubmit} enctype="multipart/form-data">
                        <div class="modal-header">
                            <h5 class="modal-title">{format!("{id_prefix} Letter Pair")}</h5>
                            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                        </div>
                        <div class="modal-body">
                            <div class="mb-3">
                                <label for={format!("{id_prefix}InputPair")} class="form-label">{"Pair"}</label>
                                <input type="text" class="form-control" id={format!("{id_prefix}InputPair")} name="InputPair" placeholder="AB"
                                    pattern=".{2,2}" title="Please input 2 characters." required=true readonly={is_modify} />
                            </div>
                            <div class="mb-3">
                                <label for={format!("{id_prefix}InputObject")} class="form-label">{"Object"}</label>
                                <input type="text" class="form-control" id={format!("{id_prefix}InputObject")} name="InputObject" placeholder="ABS"
                                    pattern=".{1,32}" title="Please input 32 characters or less." required=true />
                            </div>
                            <div class="mb-3">
                                <label for={format!("{id_prefix}InputImage")} class="form-label">{"Image"}</label>
                                <input class="form-control" type="file" id={format!("{id_prefix}InputImage")} name="InputImage" accept="image/*" aria-describedby="imageHelp" />
                                <div id="imageHelp" class="form-text">{"Images are cropped to a maximum of 256x256."}</div>
                            </div>
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Close"}</button>
                            <button type="submit" class="btn btn-primary" data-bs-dismiss="modal">{"Save"}</button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    }
}
