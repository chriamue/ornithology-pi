use yew::prelude::*;

use crate::about::About;
use crate::contexts::ApiUrlProvider;
use crate::footer::Footer;
use crate::header::Header;
use crate::server_input::ServerInput;
use crate::sightings::SightingsContainer;
use crate::webcam::Webcam;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div id="app">
        <ApiUrlProvider>
            <Header />
            <ServerInput />
            <Webcam />
            <SightingsContainer />
            <About />
            <Footer />
        </ApiUrlProvider>
        </div>
    }
}
