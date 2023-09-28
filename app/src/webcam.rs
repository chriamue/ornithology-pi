use yew::prelude::*;
use crate::contexts::ApiUrl;
use crate::contexts::ApiUrlContext;

pub enum Msg {
    Click,
}

enum Source {
    Webcam,
    Frame,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub api_url: Option<String>,
}

pub struct Webcam {
    source: Source,
}

impl Component for Webcam {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            source: Source::Frame,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Click => {
                self.source = match self.source {
                    Source::Frame => Source::Webcam,
                    Source::Webcam => Source::Frame,
                };
                true
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::Click);
        let mut base_url = ctx.props().api_url.as_ref().unwrap_or(&"".to_string()).to_string();
        if !base_url.ends_with("/") {
            base_url.push_str("/");
        }

        match self.source {
            Source::Frame => html! {
                <div class="row card justify-content-center d-grid gap-3">
                    <img id="webcam" src={format!("{}frame", base_url)} title="webcam" {onclick} />
                </div>
            },
            Source::Webcam => html! {
                <div class="row card justify-content-center d-grid gap-3">
                    <img id="webcam" src={format!("{}webcam", base_url)} title="webcam" {onclick} />
                </div>
            },
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}
}

#[function_component()]
pub fn WebcamContainer() -> Html {
    let api_url: String = match use_context::<ApiUrlContext>() {
        Some(api_url) => api_url.inner.clone(),
        None => ApiUrl::default().inner,
    };

    html! {
        <div class="container">
            <Webcam api_url={api_url} />
        </div>
    }
}
