use yew::prelude::*;

pub struct App {
    link: ComponentLink<Self>,
    count: i32,
}
pub enum Msg {
    Add,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link: link,
            count: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match (msg) {
            Msg::Add => {
                self.count += 1;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            <p>{"Count = "}{ self.count }
            <br />
            <button onclick=self.link.callback(|_| Msg::Add) >{"+1"}</button>
            </p>
            </>
        }
    }
}
