use yew::{Component, ComponentLink, Html, Properties, html};
use yew::html::Scope;

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct ArticleProps {
    pub id: u32
}

#[derive(Debug)]
pub struct Article {
    pub props: ArticleProps,
    pub link: Scope<Article>
}

impl Component for Article {
    type Message = ();
    type Properties = ArticleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Article { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        unimplemented!()
    }

    fn view(&self) -> Html {
        html! {
            <div class="main-content">
                <div>{format!("文章id: {}", self.props.id)}</div>
            </div>
        }
    }
}
