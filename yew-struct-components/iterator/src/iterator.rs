struct TableRow {
    id: i32,
    name: String,
    admin: bool,
}

pub struct App {
    table: Vec<TableRow>,
}

pub enum Msg {}

impl yew::Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &yew::Context<Self>) -> Self {
        App {
            table: vec![
                TableRow {
                    id: 0,
                    name: "Alice".into(),
                    admin: true,
                },
                TableRow {
                    id: 1,
                    name: "Bob".into(),
                    admin: true,
                },
                TableRow {
                    id: 2,
                    name: "Charles".into(),
                    admin: false,
                },
            ],
        }
    }

    fn update(&mut self, _: &yew::Context<Self>, _: Self::Message) -> bool {
        true
    }

    fn view(&self, _: &yew::Context<Self>) -> yew::Html {
        use yew::html;
        html! {
            <>
            <p>{ "Iterator Type 1" }</p>
            <table frame="box" rules="all">
            <tr><th>{ "ID" }</th><th>{ "name" }</th></tr>{
                self.table.iter().map(|row| {
                    html! {<tr><td>{row.id}</td><td>{&row.name}</td></tr>}
                }).collect::<yew::Html>()
            }
            </table>
            <p>{ "Iterator Type 2" }</p>
            <table frame="box" rules="all">
            <tr><th>{ "ID" }</th><th>{ "name" }</th></tr>{
                for self.table.iter().map(|row| {
                    html! {<tr><td>{row.id}</td><td>{&row.name}</td></tr>}
                })
            }
            </table>
            <p>{ "Iterator with condition" }</p>
            <table frame="box" rules="all">
            <tr><th>{ "ID" }</th><th>{ "admin name" }</th></tr>{
                for self.table.iter().map(|row| {
                    if row.admin {
                        html! {<tr><td>{row.id}</td><td>{&row.name}</td></tr>}
                    }else{
                        html! {}
                    }
                })
            }
            </table>
            </>
        }
    }
}
