use chrono::Utc;
use chrono::{DateTime, NaiveDateTime};
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_bootstrap::component::*;
use yew_bootstrap::util::Color;

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Sighting {
    pub uuid: String,
    pub timestamp: i64,
    pub species: String,
}

#[derive(Clone, Properties, PartialEq)]
pub struct SightingProps {
    pub api_url: Option<String>,
    pub sighting: Sighting,
}

#[function_component(SightingDetails)]
pub fn sighting_details(SightingProps { api_url, sighting }: &SightingProps) -> Html {
    let datetime =
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(sighting.timestamp, 0), Utc);
    let mut base_url = api_url.as_ref().unwrap_or(&"".to_string()).to_string();
    if !base_url.ends_with("/") {
        base_url.push_str("/");
    }

    let uuid = sighting.uuid.clone();
    html! {
        <div class="col-md-4 card">
            <h3>{ sighting.species.clone() }</h3>
            <p>{datetime}</p>
            <img src={format!("{}sightings/{}", base_url, sighting.uuid.clone())} title={sighting.species.clone()} />
            <Button style={Color::Danger} onclick={Callback::from(move |_| {
                let uuid = uuid.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    reqwest::Client::new().delete(&format!("/sightings/{}", uuid))
                            .send()
                            .await
                            .unwrap();
                });
            })}>{"X"}</Button>
        </div>
    }
}
