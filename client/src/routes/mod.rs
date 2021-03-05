use yew_router::{Switch, router::Router};
use yew::prelude::{Html, html};

use crate::views::{
    index::Index,
    info::Info,
    article::Article
};

#[derive(Switch, Debug, Clone, PartialEq, Copy)]
pub enum AppRoute {
    #[to="/article/{id}"]
    Article(u32),
    #[to = "/info"]
    Info,
    #[to = "/"]
    Index,
}

pub fn router() -> Html {
    html! {
        <Router<AppRoute, ()>
            render = Router::render(|route: AppRoute| {
                match route {
                    AppRoute::Article(id) => html!{ <Article id=id /> },
                    AppRoute::Info => html!{ <Info /> },
                    AppRoute::Index => html!{ <Index /> },
                }
            })
        />
    }
}
