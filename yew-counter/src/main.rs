mod counter;

// Use wee_alloc to reduce WASM binary size
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn main() {
    yew::start_app::<counter::App>();
}
