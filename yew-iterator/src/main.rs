#![recursion_limit = "512"]
mod iterator;

pub fn main() {
    yew::Renderer::<iterator::App>::new().render();
}
