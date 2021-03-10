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
use yew::services::{FetchService, ConsoleService, fetch::FetchTask};
use http::{Request,Response};
use yew::format::Json;
use http::Error;
use serde::{Deserialize, Serialize};

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct IndexProps {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiInfo {
    version: f32,
    server: String,
    deps: Vec<String>,
}

pub enum Msg {
    FetchInfo,
    FetchComplete(ApiInfo),
    FetchError(String),
}

#[derive(Debug)]
pub struct Index {
    props: IndexProps,
    link: ComponentLink<Self>,
    info: Option<ApiInfo>,
    fetch_task: Option<FetchTask>
}

impl Component for Index {
    type Message = Msg;
    type Properties = IndexProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            info: None,
            fetch_task: None
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        // CORS ERROR？
        if _first_render {
            let task = FetchService::fetch(
                Request::get("http://127.0.0.1:9999/getArticles")
                    .header("Content-Type", "application/json")
                    .body(Nothing)
                    .expect("Failed to build request.")
                ,
                self.link.callback(|res: Response<Result<String, _>>| {
                    if res.status().is_success() {
                        ConsoleService::info("Fetch Success");
                        let body = res.body().as_ref().unwrap();
                        let api_info: ApiInfo = serde_json::from_str(&body).unwrap();
                        Msg::FetchComplete(api_info)
                    } else {
                        ConsoleService::info("Fetch Error");
                        Msg::FetchError(String::from("Fetch Failed"))
                    }
                })
            ).unwrap();

            self.fetch_task = Some(task)
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
