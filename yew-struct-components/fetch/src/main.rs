#![recursion_limit = "256"]
mod fetch;

pub fn main() {
    yew::Renderer::<fetch::App>::new().render();
}
