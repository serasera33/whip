mod app;
mod router;
mod home;
mod login;
mod components;
mod messenger;
mod header;

use zoon::*;

#[wasm_bindgen(start)]
pub fn start() {
    zoon::println!("Starting APP");
    router::router();
    start_app("app", app::root);
}