mod helloworld;

pub fn main() {
    yew::Renderer::<helloworld::App>::new().render();
}
