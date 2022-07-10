use yew::prelude::*;

#[function_component(Header)]
pub fn comp() -> Html {
    html! {
        <div class="jumbotron mt-4 p-3 mb-5 bg-light rounded shadow">
            <h1>{"Ornithology PI"}</h1>
        </div>
    }
}
