use yew::prelude::*;
use crate::components::card::Card;

struct Pair {
    initial: String,
    next: String,
    object: String,
    image: String,
}

#[function_component(Top)]
pub fn top() -> Html {
    let pairs = vec![
        Pair {
            initial: "あ".to_string(),
            next: "い".to_string(),
            object: "アイス".to_string(),
            image: "https://avatars1.githubusercontent.com/u/29778890?s=400&v=4".to_string(),
        },
        Pair {
            initial: "あ".to_string(),
            next: "う".to_string(),
            object: "アウディ".to_string(),
            image: "https://avatars1.githubusercontent.com/u/29778890?s=400&v=4".to_string(),
        }
    ];

    let pairs: Vec<Html> = pairs.iter()
        .map(|pair| html! {
            <Card pair={format!("{}{}", &pair.initial, &pair.next)} object={pair.object.clone()} img={pair.image.clone()} />
        })
        .collect();

    html! {
        <div class="contents">
            <h2 class="underline" id="あ">{"あ"}</h2>
            <div class="pairs">
                <Card pair={"あい"} object={"アイス"} img={"https://avatars1.githubusercontent.com/u/29778890?s=400&v=4"} />
                <Card pair={"あう"} object={"アウディ"} img={"https://avatars1.githubusercontent.com/u/29778890?s=400&v=4"} />
                <Card pair={"あえ"} object={"亜鉛"} img={"https://avatars1.githubusercontent.com/u/29778890?s=400&v=4"} />
            </div>
            <h2 class="underline" id="い">{"い"}</h2>
            <div class="pairs">
                <Card pair={"いあ"} object={"IA"} img={"https://avatars1.githubusercontent.com/u/29778890?s=400&v=4"} />
                <Card pair={"いう"} object={"言う"} img={"https://avatars1.githubusercontent.com/u/29778890?s=400&v=4"} />
                <Card pair={"いえ"} object={"遺影"} img={"https://avatars1.githubusercontent.com/u/29778890?s=400&v=4"} />
            </div>
            <h2 class="underline" id="develop">{"Develop"}</h2>
            <div class="pairs">
                {pairs}
            </div>
        </div>
    }
}
