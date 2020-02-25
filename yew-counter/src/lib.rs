mod counter;

use wasm_bindgen::prelude::*;

// Use wee_alloc to reduce WASM binary size
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<counter::App>();
    Ok(())
}
