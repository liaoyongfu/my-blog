use yew::{
    Component,
    ComponentLink,
    Properties,
    ShouldRender,
    Html,
    html,
};

use yew_router::{
    components::RouterAnchor,
    service::RouteService,
    agent::RouteAgentBridge,
    route::Route,
};

use crate::routes::AppRoute;

#[derive(Debug, Clone, PartialEq)]
pub struct MenuItem {
    pub(crate) name: &'static str,
    pub(crate) url: &'static str,
    pub(crate) router: AppRoute
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct RouterNavProps {
    pub menu: Vec<MenuItem>
}

pub struct RouterNav {
    _link: ComponentLink<Self>,
    props: RouterNavProps,
    current_route: Route,
    _route_agent: RouteAgentBridge,
}

pub enum NavMsg {
    RouteChange(Route<()>),
}

type RouteLink = RouterAnchor<AppRoute>;

impl Component for RouterNav {
    type Message = NavMsg;
    type Properties = RouterNavProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let update_route = link.callback(NavMsg::RouteChange);
        let route_agent = RouteAgentBridge::new(update_route);
        let current_route = RouteService::new().get_route();

        Self {
            _link: link,
            props,
            current_route,
            _route_agent: route_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            NavMsg::RouteChange(route) => {
                self.current_route = route;
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <nav role="navigation">
                <ul role="menubar">
                    {
                        self.props.menu.iter().map(|item| {
                            html! {
                                <li role="menuitem" class=self.active_classname(item.url)>
                                    <RouteLink route=item.router>{ item.name }</RouteLink>
                                </li>
                             }
                        }).collect::<Html>()
                    }
                </ul>
            </nav>
        }
    }
}

impl RouterNav {
    fn active_classname(&self, route: &str) -> &str {
        match &self.current_route.route {
            curr if curr == route => "selected",
            _ => "not-selected"
        }
    }
}
