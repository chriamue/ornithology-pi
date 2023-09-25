use yew::prelude::*;

pub enum Msg {
    Click,
}

enum Source {
    Webcam,
    Frame,
}

pub struct Webcam {
    source: Source,
}

impl Component for Webcam {
    type Message = Msg;
    type Properties = ();

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

        match self.source {
            Source::Frame => html! {
                <div class="row card justify-content-center d-grid gap-3">
                    <img id="webcam" src="/frame" title="webcam" {onclick} />
                </div>
            },
            Source::Webcam => html! {
                <div class="row card justify-content-center d-grid gap-3">
                    <img id="webcam" src="/webcam" title="webcam" {onclick} />
                </div>
            },
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}
}
