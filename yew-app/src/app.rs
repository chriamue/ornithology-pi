use yew::prelude::*;

use crate::about::About;
use crate::footer::Footer;
use crate::header::Header;
use crate::sightings::Sightings;
use crate::webcam::Webcam;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div id="app">
        <Header />
        <Webcam />
        <Sightings />
        <About />
        <Footer />
        </div>
    }
}
