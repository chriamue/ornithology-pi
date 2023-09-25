use std::rc::Rc;
use web_sys::Url;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ApiUrl {
    pub inner: String,
}

impl Default for ApiUrl {
    fn default() -> Self {
        let protocol = web_sys::window()
            .unwrap()
            .location()
            .protocol()
            .unwrap_or_else(|_| String::from("http:"));
        let host = web_sys::window()
            .unwrap()
            .location()
            .host()
            .unwrap_or_else(|_| String::from("localhost:8080"));
        Self {
            inner: format!("{}//{}/", protocol, host),
        }
    }
}

impl Reducible for ApiUrl {
    type Action = String;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        ApiUrl { inner: action }.into()
    }
}

pub type ApiUrlContext = UseReducerHandle<ApiUrl>;

#[derive(Properties, Debug, PartialEq)]
pub struct ApiUrlProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn ApiUrlProvider(props: &ApiUrlProviderProps) -> Html {
    let reducer = use_reducer(|| ApiUrl::default().into());

    html! {
        <ContextProvider<ApiUrlContext> context={reducer}>
            {props.children.clone()}
        </ContextProvider<ApiUrlContext>>
    }
}
