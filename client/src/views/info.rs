use yew::{
    Component,
    ComponentLink,
    ShouldRender,
    Html,
    html,
    format::Nothing,
};
use yew::services::{ConsoleService, FetchService};
use yew::services::fetch::{Request, Response, FetchTask};
use serde::{Serialize, Deserialize};
use dotenv_codegen::dotenv;

#[derive(Debug)]
pub struct Info {
}

impl Component for Info {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="main-content">
                <p>{"关于"}</p>
            </div>
        }
    }
}
