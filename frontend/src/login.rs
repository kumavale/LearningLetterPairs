use gloo_net::http::Request;
use serde::Serialize;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlFormElement, FormData};
use yew::prelude::*;

#[derive(Debug, Serialize)]
struct Credentials {
    username: String,
    password: String,
}

#[function_component(Login)]
pub fn login() -> Html {
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        let target: Option<EventTarget> = e.target();
        let form: HtmlFormElement = target.and_then(|t| t.dyn_into::<HtmlFormElement>().ok()).unwrap();
        let form_data = FormData::new_with_form(&form).unwrap();
        let credentials = Credentials {
            username: form_data.get("input-username").as_string().unwrap_or_default(),
            password: form_data.get("input-password").as_string().unwrap_or_default(),
        };

        wasm_bindgen_futures::spawn_local(async move {
            match Request::post(&format!("{}/login", crate::BACKEND_URL))
                .credentials(web_sys::RequestCredentials::Include)
                //.mode(web_sys::RequestMode::Cors)
                .json(&credentials)
                .unwrap()
                .send()
                .await
            {
                Ok(res) => {
                    match res.status() {
                        // ログイン成功
                        200 => {
                            log::info!("login success");
                            // トップページへ推移
                            web_sys::window().unwrap().location().set_href("/").ok();
                        }
                        // ログイン失敗
                        n => {
                            log::error!("login failed: [{n}]");
                        }
                    }
                }
                // TODO: POST失敗
                Err(e) => {
                    log::error!("{:?}", e);
                }
            }
        });

        e.prevent_default();
    });

    html! {
        <div class="contents">
            <div class="form-login">
                <form onsubmit={onsubmit} style="width: 100%;">
                    // Username input
                    <div class="form-group mb-4">
                        <label class="form-label" for="input-username">{"Username"}</label>
                        <input type="text" id="input-username" name="input-username" class="form-control" placeholder="Username" required=true />
                    </div>
                    // Password input
                    <div class="form-group mb-4">
                        <label class="form-label" for="input-password">{"Password"}</label>
                        <input type="password" id="input-password" name="input-password" class="form-control" placeholder="Password" required=true autocomplete="on" />
                    </div>
                    // Checkbox
                    <div class="form-group mb-4">
                        <div class="form-check">
                            <label class="form-check-label" for="input-remember">{"Remember me"}</label>
                            <input class="form-check-input" type="checkbox" value="" id="input-remember" checked=true />
                        </div>
                    </div>
                    // Submit button
                    <button type="submit" class="btn btn-primary btn-block mb-4" style="width: 100%;">{"Login"}</button>
                    // Register buttons
                    <div class="text-center">
                        <p>{"Not a member? "}<a href="#!">{"Register"}</a></p>
                    </div>
                </form>
            </div>
        </div>
    }
}
