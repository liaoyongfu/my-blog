use yew::{
    Component,
    ComponentLink,
    Properties,
    ShouldRender,
    Html,
    html,
};
use crate::components::list::List;
use crate::components::list::ListItem;

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct IndexProps {}

pub enum Msg {}

#[derive(Debug)]
pub struct Index {
    props: IndexProps,
    link: ComponentLink<Self>,
}

impl Component for Index {
    type Message = Msg;
    type Properties = IndexProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
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
                url: "/article/1",
                brief: "这个是文章的简介这个是文章的简介这个是文章的简介这个是文章的简介这个是文章的简介这个是文章的简介这个是文章的简介这个是文章的简介..."
            },
            ListItem {
                title: "文章标题",
                date: "2021-3-2 17:00:17",
                url: "/article/2",
                brief: "这个是文章的简介这个是文章的简介这个是文章的简介这个是文章的简介这个是文章的简介这个是文章的简介这个是文章的简介这个是文章的简介..."
            }
        ];
        html! {
            <div id="index-view" style="width:100%;">
                <div class="main-content">
                    <List data=data />
                </div>
            </div>
        }
    }
}
