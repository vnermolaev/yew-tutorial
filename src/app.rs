use crate::fetch::{get_ip_info, HostIPResponse};
use anyhow::Error;
use rand::Rng;
use yew::format::Json;
use yew::services::fetch::FetchTask;
use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Model {
    items: Vec<i64>,
    link: ComponentLink<Self>,
    ip: Option<String>,
    task: Option<FetchTask>,
}

pub enum Msg {
    AddOne,
    RemoveOne,
    GetIpResponse,
    IpResponseReady(Result<HostIPResponse, Error>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            items: Vec::new(),
            link,
            ip: None,
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let mut rng = rand::thread_rng();

        match msg {
            Msg::AddOne => {
                let item = rng.gen_range(1..=6);
                ConsoleService::debug(format!("Adding {}", item).as_str());
                self.items.push(item);
            }
            Msg::RemoveOne => {
                if let Some(item) = self.items.pop() {
                    ConsoleService::debug(format!("Removing {}", item).as_str());
                } else {
                    ConsoleService::error("List is empty");
                }
            }

            Msg::GetIpResponse => {
                self.task = Some(get_ip_info(self.link.callback(Msg::IpResponseReady)));
            }

            Msg::IpResponseReady(Ok(r)) => {
                self.ip = Some(r.ip.clone());
                ConsoleService::info(format!("Response: {:?}", Json(r)).as_str());
            }
            Msg::IpResponseReady(Err(err)) => {
                ConsoleService::error(format!("Error: {:?}", err).as_str());
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
                <h1>{ format!("{:?}", self.ip ) }</h1>
                <button onclick=self.link.callback(|_| Msg::GetIpResponse)>{ "Get IP" }</button>
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
