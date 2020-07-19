use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::fetch;

pub struct App {
    link: ComponentLink<Self>,
    selected: i32,
    message: String,
    fetch: Option<fetch::FetchTask>, // keep FetchTask not to be dropped during task running
}
pub enum Msg {
    Select(i32),
    StartFetch,
    FinishFetch(fetch::Response<Result<String, anyhow::Error>>),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link: link,
            selected: 0,
            message: "Nothing are fetched yet.".to_string(),
            fetch: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Select(selected) => {
                self.selected = selected;
                true
            }
            Msg::StartFetch => {
                let path = if 0 == self.selected {
                    "/static/red.json"
                } else {
                    "/static/green.json"
                };
                let req = fetch::Request::get(path)
                    .body(yew::format::Nothing)
                    .unwrap();
                let fetchresult = fetch::FetchService::fetch(
                    req,
                    self.link.callback(|response| Msg::FinishFetch(response)),
                );
                match fetchresult {
                    Ok(task) => {
                        self.fetch = Some(task);
                        self.message = "Fetcing...".to_string();
                    }
                    Err(err) => {
                        self.fetch = None;
                        self.message = err.to_string();
                    }
                }
                true
            }
            Msg::FinishFetch(response) => {
                ConsoleService::debug(&format!("Fetch response = {:?}", response));
                match response.body() {
                    Ok(body) => self.message = body.clone(),
                    Err(err) => self.message = err.to_string(),
                }
                self.fetch = None;
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
            <input type="radio" id="selector0" name="selector" value="0" checked=(0==self.selected) onchange=self.link.callback(move |_| Msg::Select(0)) />
            <label for="selector0">{" red.json" }</label>
            <input type="radio" id="selector1" name="selector" value="1" checked=(1==self.selected) onchange=self.link.callback(move |_| Msg::Select(1)) />
            <label for="selector1">{" green.json" }</label>
            <br />
            <button onclick=self.link.callback(move |_| Msg::StartFetch)>{"Fetch"}</button>
            <br />
            <p>{ &self.message }</p>
            </>
        }
    }
}
