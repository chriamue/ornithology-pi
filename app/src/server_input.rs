use crate::contexts::ApiUrlContext;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yew_bootstrap::{
    component::{Button, Container, ContainerSize},
    util::Color,
};

#[function_component(ServerInput)]
pub fn server_input() -> Html {
    let api_url = use_context::<ApiUrlContext>().unwrap();
    let input_value = api_url.inner.clone();

    let on_input = {
        let api_url = api_url.clone();
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                api_url.dispatch(input.value());
            }
        })
    };

    html! {
        <div>
        <Container class="bg-primary" size={ContainerSize::Medium}>
            <input type="text" value={input_value.clone()} onchange={on_input} placeholder="Enter server URL" />
            <Button style={Color::Secondary} onclick={Callback::from(move |_| {
                web_sys::console::log_1(&format!("Server URL set to: {}", api_url.inner).into());
            })}>
                {"Set Server URL"}
            </Button>
        </Container>
        </div>
    }
}
