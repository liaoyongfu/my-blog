use yew::{
    Component,
    ComponentLink,
    Properties,
    ShouldRender,
    Html,
    html,
    format::Nothing,
    prelude::*
};
use crate::components::list::List;
use crate::components::list::ListItem;
use crate::routes::AppRoute;
use yew::services::{FetchService, ConsoleService};
use http::{Request,Response};
use yew::format::Json;
use http::Error;
use serde::Deserialize;

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct IndexProps {}

pub enum Msg {}

#[derive(Debug)]
pub struct Index {
    props: IndexProps,
    link: ComponentLink<Self>,
}

#[derive(Deserialize)]
struct Data {
    code: u32
}

impl Component for Index {
    type Message = ();
    type Properties = IndexProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        // TODO fetch canceld, why ?
        if _first_render {
            FetchService::fetch(
                Request::get("http://127.0.0.1:9999/getArticles")
                    .header("Content-Type", "application/json")
                    .body(Nothing)
                    .unwrap()
                ,
                self.link.callback(|res: Response<Result<String, _>>| {
                    ConsoleService::log("Success: ");
                    ConsoleService::log(res.status().as_str());
                })
            );
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let data: Vec<ListItem> = vec![
            ListItem {
                title: "文章标题",
                date: "2021-3-2 17:00:17",
                brief: "这个是文章的简介这个是文章的简介这个是文章的简介这个是文章的简介这个是文章的简介这个是文章的简介这个是文章的简介这个是文章的简介...",
                router: AppRoute::Article(1)
            },
            ListItem {
                title: "文章标题",
                date: "2021-3-2 17:00:17",
                brief: "这个是文章的简介这个是文章的简介这个是文章的简介这个是文章的简介这个是文章的简介这个是文章的简介这个是文章的简介这个是文章的简介...",
                router: AppRoute::Article(2)
            }
        ];
        html! {
            <div id="index-view" style="width:100%">
                <div class="main-content">
                    <List data=data />
                </div>
            </div>
        }
    }
}
