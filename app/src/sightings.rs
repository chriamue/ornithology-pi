use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use yew::prelude::*;
use yew_bootstrap::component::Button;
use yew_bootstrap::util::Color;

use crate::contexts::{ApiUrl, ApiUrlContext};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Sighting {
    pub uuid: String,
    pub timestamp: i64,
    pub species: String,
}

#[derive(Clone, Properties, PartialEq)]
struct SightingProps {
    api_url: Option<String>,
    sighting: Sighting,
}

#[function_component(SightingDetails)]
fn sighting_details(SightingProps { api_url, sighting }: &SightingProps) -> Html {
    let datetime =
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(sighting.timestamp, 0), Utc);

    let uuid = sighting.uuid.clone();
    html! {
        <div class="col-md-4">
            <div class="card">
                <h3>{ sighting.species.clone() }</h3>
                <p>{datetime}</p>
                <img src={format!("{}/sightings/{}", api_url.as_ref().unwrap_or(&"".to_string()), sighting.uuid.clone())} title={sighting.species.clone()} />
                <button title="remove" class="btn btn-danger" onclick={Callback::from(move |_| {
                    let uuid = uuid.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        reqwest::Client::new().delete(&format!("/sightings/{}", uuid))
                                .send()
                                .await
                                .unwrap();
                    });
                })}>{"X"}</button>
            </div>
        </div>
    }
}

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
        <div class="nav">
        <Button style={Color::Primary} onclick={onleftclick.clone()}>
        {"<-"} </Button>
        <Button style={Color::Primary} onclick={onrightclick.clone()}>{"->"}</Button>
        </div>
        <div id="images" class="row card justify-content-center d-grid gap-3">
            {details}
        </div>
        <div class="nav">
        <Button style={Color::Primary} onclick={onleftclick}>
        {"<-"} </Button>
        <Button style={Color::Primary} onclick={onrightclick}>{"->"}</Button>
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
