use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
};

use crate::routes::{router, AppRoute};
use crate::components::nav::{RouterNav, MenuItem};

pub struct App {
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let menu: Vec<MenuItem> = vec![
            MenuItem {
                name: "Home",
                url: "/",
                router: AppRoute::Index
            },
            MenuItem {
                name: "Info",
                url: "/info",
                router: AppRoute::Info
            }
        ];
        html! {
            <main id="app">
                <RouterNav menu=menu />
                { router() }
            </main>
        }
    }
}

