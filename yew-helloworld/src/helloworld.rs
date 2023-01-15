pub struct App {}
pub enum Msg {}

impl yew::Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &yew::Context<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _: &yew::Context<Self>, _: Self::Message) -> bool {
        true
    }

    fn view(&self, _: &yew::Context<Self>) -> yew::Html {
        use yew::html;
        html! {
            <>
            <p>{ "Hello world!" }</p>
            </>
        }
    }
}
