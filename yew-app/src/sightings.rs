use chrono::{DateTime, NaiveDateTime, Utc};
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use yew::prelude::*;

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Sighting {
    pub uuid: String,
    pub timestamp: i64,
    pub species: String,
}

#[derive(Clone, Properties, PartialEq)]
struct SightingProps {
    sighting: Sighting,
}

#[function_component(SightingDetails)]
fn sighting_details(SightingProps { sighting }: &SightingProps) -> Html {
    let datetime =
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(sighting.timestamp, 0), Utc);

    let uuid = sighting.uuid.clone();
    html! {
        <div>
            <h3>{ sighting.species.clone() }</h3>
            <p>{datetime}</p>
            <img src={format!("/sightings/{}", sighting.uuid.clone())} title={sighting.species.clone()} />
            <button title="remove" class="btn btn-danger" onclick={Callback::from(move |_| {
                let uuid = uuid.clone();
                wasm_bindgen_futures::spawn_local(async move {
                        Request::delete(&format!("/sightings/{}", uuid))
                            .send()
                            .await
                            .unwrap();
                });
            })}>{"X"}</button>
        </div>
    }
}

pub enum Msg {
    ClickLeft,
    ClickRight,
}

pub struct Sightings {
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
        wasm_bindgen_futures::spawn_local(async move {
            let fetched: Vec<Sighting> =
                Request::get(&format!("/sightings?start={}&end={}", start, end))
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
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut created = Self {
            start: 0,
            end: 9,
            sightings: Arc::new(Mutex::new(vec![])),
        };
        created.fetch();
        created
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
                  <SightingDetails sighting={sighting.clone()} />
                }
            });
            details.collect()
        };

        html! {
                    <>
        <div class="nav">
        <button class="btn btn-primary" onclick={onleftclick.clone()}>
        {"<-"} </button>
        <button class="btn btn-primary" onclick={onrightclick.clone()}>{"->"}</button>
        </div>
        <div id="images" class="row card justify-content-center d-grid gap-3">
            {details}
        </div>
        <div class="nav">
        <button class="btn btn-primary" onclick={onleftclick}>
        {"<-"} </button>
        <button class="btn btn-primary" onclick={onrightclick}>{"->"}</button>
        </div>
                    </>
                }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}
}
