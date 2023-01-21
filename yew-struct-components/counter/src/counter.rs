pub struct App {
    count: i32,
}
pub enum Msg {
    Add,
}

impl yew::Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &yew::Context<Self>) -> Self {
        App { count: 0 }
    }

    fn update(&mut self, _: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add => {
                self.count += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        use yew::html;
        html! {
            <>
            <p>{"Count = "}{ self.count }
            <br />
            <button onclick={ ctx.link().callback(|_| Msg::Add) }>{"+1"}</button>
            </p>
            </>
        }
    }
}
