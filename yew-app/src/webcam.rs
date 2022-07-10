use yew::prelude::*;

#[function_component(Webcam)]
pub fn comp() -> Html {
    html! {
        <div class="row card justify-content-center d-grid gap-3">
            <img id="webcam" src="/frame" title="webcam" />
        </div>
    }
}
