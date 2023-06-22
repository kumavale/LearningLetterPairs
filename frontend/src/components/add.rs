use gloo_net::http::Request;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlFormElement, FormData};
use yew::prelude::*;
use yew::Callback;

#[function_component(AddButton)]
pub fn add_button() -> Html {
    html! {
        <button class="fixed-button" type="button" data-bs-toggle="modal" data-bs-target="#add-modal">{"\u{FF0B}"}</button>
    }
}

#[function_component(AddModal)]
pub fn add_modal() -> Html {
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        let target: Option<EventTarget> = e.target();
        let form: HtmlFormElement = target.and_then(|t| t.dyn_into::<HtmlFormElement>().ok()).unwrap();
        let form_data = FormData::new_with_form(&form).unwrap();

        wasm_bindgen_futures::spawn_local(async move {
            if let Err(_e) = Request::post("http://localhost:3000/pairs")
                .body(&form_data)
                .unwrap()
                .send()
                .await
            {
                // TODO: POST失敗
            }
        });

        //e.prevent_default();
    });

    html! {
        <div class="modal fade" id="add-modal" tabindex="-1">
            <div class="modal-dialog modal-dialog-centered">
                <div class="modal-content">
                    <form onsubmit={onsubmit} enctype="multipart/form-data">
                        <div class="modal-header">
                            <h5 class="modal-title">{"Append Pair"}</h5>
                            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                        </div>
                        <div class="modal-body">
                            <div class="mb-3">
                                <label for="InputPair" class="form-label">{"Pair"}</label>
                                <input type="text" class="form-control" id="InputPair" name="InputPair" placeholder="AB"
                                    pattern=".{2,2}" title="Please input 2 characters." required=true />
                            </div>
                            <div class="mb-3">
                                <label for="InputObject" class="form-label">{"Object"}</label>
                                <input type="text" class="form-control" id="InputObject" name="InputObject" placeholder="ABS"
                                    pattern=".{1,32}" title="Please input 32 characters or less." required=true />
                            </div>
                            <div class="mb-3">
                                <label for="InputImage" class="form-label">{"Image"}</label>
                                <input class="form-control" type="file" id="InputImage" name="InputImage" accept="image/*" aria-describedby="imageHelp" />
                                <div id="imageHelp" class="form-text">{"Images are cropped to a maximum of 256x256."}</div>
                            </div>
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Close"}</button>
                            <button type="submit" class="btn btn-primary">{"Save changes"}</button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    }
}
