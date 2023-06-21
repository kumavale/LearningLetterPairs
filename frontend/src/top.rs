use gloo_net::http::Request;
use yew::prelude::*;
use crate::components::card::Card;
use crate::types::Pair;

#[function_component(Top)]
pub fn top() -> Html {
    let pairs = use_state(Vec::new);
    {
        let pairs = pairs.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_pairs: Vec<Pair> = Request::get("http://localhost:3000/pairs")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                pairs.set(fetched_pairs);
            });
            || ()
        }, ());
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
        </div>
    }
}