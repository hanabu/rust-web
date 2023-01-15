//use yew::services::console::ConsoleService;

pub struct App {
    selected: i32,
    message: String,
}
pub enum Msg {
    Select(i32),
    StartFetch,
    FinishFetch(String),
}

impl yew::Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &yew::Context<Self>) -> Self {
        App {
            selected: 0,
            message: "Nothing are fetched yet.".to_string(),
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Select(selected) => {
                self.selected = selected;
                true
            }
            Msg::StartFetch => {
                let path = if 0 == self.selected {
                    "/red.json"
                } else {
                    "/green.json"
                };
                let req = gloo_net::http::Request::get(path);
                ctx.link().send_future(async {
                    let response = req.send().await;
                    gloo_console::log!(
                        "response:",
                        wasm_bindgen::JsValue::from(format!("{:?}", response))
                    );
                    match response {
                        Ok(resp) => match resp.text().await {
                            Ok(body) => Msg::FinishFetch(body),
                            Err(err) => Msg::FinishFetch(err.to_string()),
                        },
                        Err(err) => Msg::FinishFetch(err.to_string()),
                    }
                });
                self.message = "Fetching...".to_string();
                true
            }
            Msg::FinishFetch(resp_text) => {
                self.message = resp_text;
                true
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        use yew::html;
        html! {
            <>
            <input type="radio" id="selector0" name="selector" value="0" checked={0==self.selected} onchange={ ctx.link().callback(|_| Msg::Select(0)) } />
            <label for="selector0">{" red.json" }</label>
            <input type="radio" id="selector1" name="selector" value="1" checked={1==self.selected} onchange={ ctx.link().callback(|_| Msg::Select(1)) } />
            <label for="selector1">{" green.json" }</label>
            <br />
            <button onclick={ ctx.link().callback(|_| Msg::StartFetch) }>{"Fetch"}</button>
            <br />
            <p>{ &self.message }</p>
            </>
        }
    }
}
