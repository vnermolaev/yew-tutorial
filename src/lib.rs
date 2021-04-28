use crate::app::Model;
use wasm_bindgen::prelude::*;
use yew::prelude::App;

mod app;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
