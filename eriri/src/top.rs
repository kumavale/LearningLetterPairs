use crate::components::{
    add::{AddButton, AddModal},
    card::Card,
};
use crate::types::Pair;
use gloo_net::http::Request;
use yew::prelude::*;

#[function_component(Top)]
pub fn top() -> Html {
    let pairs = use_state(Vec::new);
    {
        let pairs = pairs.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match Request::get(&format!("{}/pairs", crate::BACKEND_URL))
                    .credentials(web_sys::RequestCredentials::Include)
                    .send()
                    .await
                {
                    Ok(res) => {
                        match res.status() {
                            // 成功
                            200 => {
                                let fetched_pairs: Vec<Pair> = res.json().await.unwrap();
                                pairs.set(fetched_pairs);
                            }
                            // 要ログイン
                            401 => {
                                web_sys::window()
                                    .unwrap()
                                    .location()
                                    .set_href("/login")
                                    .unwrap();
                            }
                            // TODO: その他
                            n => {
                                log::error!("error: http status({})", n);
                            }
                        }
                    }
                    Err(e) => {
                        // TODO: GET失敗
                        log::error!("{:?}", e);
                    }
                }
            });
            || ()
        });
    }

    let pairs: Vec<Html> = pairs.iter()
        .map(|pair| html! {
            <Card pair={format!("{}{}", &pair.initial, &pair.next)} object={pair.object.clone()} img={pair.image.clone()} />
        })
        .collect();

    html! {
        <div class="contents">
            // TODO: 1文字目でグループ化 → <h2 class="underline" id="あ">{"あ"}</h2>
            <div class="pairs">
                {pairs}
            </div>
            <AddModal />
            <AddModal modify=true />
            <AddButton />
        </div>
    }
}
