use yew::prelude::*;
use yew_bootstrap::component::*;
use yew_bootstrap::util::*;

use crate::about::About;
use crate::contexts::ApiUrlProvider;
use crate::footer::Footer;
use crate::header::Header;
use crate::server_input::ServerInput;
use crate::sightings::SightingsContainer;
use crate::webcam::WebcamContainer;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div id="app">
        <ApiUrlProvider>
            <Header />
            <ServerInput />
            <Line style={Color::Primary} />
            <WebcamContainer />
            <Line style={Color::Primary} />
            <SightingsContainer />
            <Line style={Color::Primary} />
            <About />
            <Footer />
        </ApiUrlProvider>
        </div>
    }
}
