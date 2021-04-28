use rand::Rng;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Model {
    items: Vec<i64>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    AddOne,
    RemoveOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            items: Vec::new(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let mut rng = rand::thread_rng();

        match msg {
            Msg::AddOne => self.items.push(rng.gen_range(1..=6)),
            Msg::RemoveOne => {
                self.items.pop();
            }
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let render_item = |item| html! { <tr><td>{ item }</td></tr> };

        html! {
            <div class="main">
                <div class="card">
                    <header>
                        {"Items: "}
                    </header>
                    <div class="card-body">
                        <table class="primary">
                            { for self.items.iter().map(render_item) }
                        </table>
                    </div>
                    <footer>
                        <button onclick=self.link.callback(|_| Msg::AddOne)>{ "Add 1" }</button> {" "}
                        <button onclick=self.link.callback(|_| Msg::RemoveOne)>{ "Remove 1" }</button>
                    </footer>
                </div>
            </div>
        }
    }
}
