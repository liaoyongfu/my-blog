use yew_router::components::{Props, RouterAnchor};
use yew::{Component, ComponentLink, Html, Properties, html, ShouldRender};
use crate::routes::AppRoute;

#[derive(Debug, Clone, PartialEq)]
pub struct ListItem {
    // 文章标题
    pub(crate) title: &'static str,
    // 发布日期
    pub(crate) date: &'static str,
    // 简介
    pub(crate) brief: &'static str,
    // 路由地址
    pub(crate) router: AppRoute
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct ListProps {
    // 数据源
    pub data: Vec<ListItem>
}

pub struct List {
    pub props: ListProps,
    pub link: ComponentLink<Self>
}

type RouteLink = RouterAnchor<AppRoute>;

impl Component for List {
    type Message = ();
    type Properties = ListProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        List { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                {self.props.data.iter().map(|item| {
                    html! {
                        <div class="list-item">
                            <RouteLink route=item.router classes="list-item-title">{item.title}</RouteLink>
                            <div class="list-item-date">{item.date}</div>
                            <div class="list-item-brief">{item.brief}</div>
                        </div>
                    }
                }).collect::<Html>()}
            </>
        }
    }
}
