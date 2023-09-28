use crate::contexts::{ApiUrl, ApiUrlContext};
use crate::sighting::{Sighting, SightingDetails};
use std::sync::{Arc, Mutex};
use yew::prelude::*;
use yew_bootstrap::component::{Button, ButtonSize};
use yew_bootstrap::util::Color;

pub enum Msg {
    ClickLeft,
    ClickRight,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub api_url: String,
}

pub struct Sightings {
    api_url: String,
    start: u16,
    end: u16,
    sightings: Arc<Mutex<Vec<Sighting>>>,
}

impl Sightings {
    pub fn prev(&mut self) {
        self.start = (self.start as i16 - 10).max(0) as u16;
        self.end = (self.end as i16 - 10).max(0) as u16;
    }

    pub fn next(&mut self) {
        self.start = self.start + 10;
        self.end = self.end + 10;
    }

    pub fn fetch(&mut self) {
        let start = self.start;
        let end = self.end;
        let sightings = self.sightings.clone();
        let mut base_url = self.api_url.clone();
        if !base_url.ends_with("/") {
            base_url.push_str("/");
        }

        web_sys::console::log_1(
            &format!("Fetching sightings {} : {}-{}", base_url, start, end).into(),
        );
        wasm_bindgen_futures::spawn_local(async move {
            let fetched: Vec<Sighting> = reqwest::Client::new()
                .get(&format!(
                    "{}sightings?start={}&end={}",
                    base_url, start, end
                ))
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            let mut sightings_lock = sightings.try_lock().unwrap();
            *sightings_lock = fetched;
        });
    }
}

impl Component for Sightings {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let api_url = ctx.props().api_url.clone();
        let mut created = Self {
            api_url,
            start: 0,
            end: 9,
            sightings: Arc::new(Mutex::new(vec![])),
        };
        created.fetch();
        created
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if self.api_url != ctx.props().api_url {
            self.api_url = ctx.props().api_url.clone();
            self.fetch();
            true
        } else {
            false
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ClickLeft => {
                self.prev();
                self.fetch();
                true
            }
            Msg::ClickRight => {
                self.next();
                self.fetch();
                true
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onleftclick = ctx.link().callback(|_| Msg::ClickLeft);
        let onrightclick = ctx.link().callback(|_| Msg::ClickRight);

        let details: Vec<Html> = {
            let locked: Vec<Sighting> = self.sightings.try_lock().unwrap().to_vec();
            let details = locked.into_iter().map(|sighting| {
                html! {
                  <SightingDetails api_url={self.api_url.clone()} sighting={sighting.clone()} />
                }
            });
            details.collect()
        };

        html! {
        <>
        <div class="nav d-flex justify-content-between">
            <Button style={Color::Primary} size={ButtonSize::Large} onclick={onleftclick.clone()}>
            {"<-"} </Button>
            <Button style={Color::Primary} size={ButtonSize::Large} onclick={onrightclick.clone()}>{"->"}</Button>
        </div>
        <div id="images" class="container">
            <div class="row">
                {details}
            </div>
        </div>
        <div class="nav d-flex justify-content-between">
            <Button style={Color::Primary} size={ButtonSize::Large} onclick={onleftclick}>
            {"<-"} </Button>
            <Button style={Color::Primary} size={ButtonSize::Large} onclick={onrightclick}>{"->"}</Button>
        </div>
        </>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}
}

#[function_component()]
pub fn SightingsContainer() -> Html {
    let api_url: String = match use_context::<ApiUrlContext>() {
        Some(api_url) => api_url.inner.clone(),
        None => ApiUrl::default().inner,
    };

    html! {
        <div class="container">
            <Sightings api_url={api_url} />
        </div>
    }
}
