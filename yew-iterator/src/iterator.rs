use yew::prelude::*;

struct TableRow {
    id: i32,
    name: String,
    admin: bool,
}

pub struct App {
    table: Vec<TableRow>,
}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
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

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            <p>{ "Iterator Type 1" }</p>
            <table frame="box" rules="all">
            <tr><th>{ "ID" }</th><th>{ "name" }</th></tr>{
                self.table.iter().map(|row| {
                    html! {<tr><td>{row.id}</td><td>{&row.name}</td></tr>}
                }).collect::<Html>()
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
                    if( row.admin ){
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
