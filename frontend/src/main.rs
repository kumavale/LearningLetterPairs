mod app;
mod components;
mod login;
mod top;
mod types;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
