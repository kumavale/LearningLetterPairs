use yew::prelude::*;
use crate::components::card::Card;

#[function_component(Top)]
pub fn top() -> Html {
    html! {
        <div class="contents">
            <div class="pairs">
                <Card pair={"あい"} object={"アイス"} img={"https://res.cloudinary.com/kumavale/image/upload/w_1024/v1660699986/CatGallery/202108291051_xhjkkm.jpg"} />
                <Card pair={"あう"} object={"アウディ"} img={"https://res.cloudinary.com/kumavale/image/upload/w_1024/v1660699986/CatGallery/202108291051_xhjkkm.jpg"} />
            </div>
        </div>
    }
}
