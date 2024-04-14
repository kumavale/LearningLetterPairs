mod app;
mod components;
mod login;
mod top;
mod types;

use app::App;

pub const BACKEND_URL: &str = "http://localhost:3000";

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
