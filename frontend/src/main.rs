mod app;
mod components;
mod login;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
