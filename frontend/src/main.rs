mod app;
mod components;
mod login;
mod top;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
