use yew::prelude::*;
use crate::components::card::Card;

#[function_component(Top)]
pub fn top() -> Html {
    html! {
        <div class="contents">
            <div class="pairs">
                <Card pair={"あい"} object={"アイス"} img={"https://avatars1.githubusercontent.com/u/29778890?s=400&v=4"} />
                <Card pair={"あう"} object={"アウディ"} img={"https://avatars1.githubusercontent.com/u/29778890?s=400&v=4"} />
                <Card pair={"あえ"} object={"亜鉛"} img={"https://avatars1.githubusercontent.com/u/29778890?s=400&v=4"} />
                <Card pair={"いあ"} object={"IA"} img={"https://avatars1.githubusercontent.com/u/29778890?s=400&v=4"} />
                <Card pair={"いう"} object={"言う"} img={"https://avatars1.githubusercontent.com/u/29778890?s=400&v=4"} />
                <Card pair={"いえ"} object={"遺影"} img={"https://avatars1.githubusercontent.com/u/29778890?s=400&v=4"} />
            </div>
        </div>
    }
}
