use yew::prelude::*;

#[function_component(Footer)]
pub fn comp() -> Html {
    html! {
    <footer class="footer">
      <div class="container">
        <p id="copyright">
          <img width="24px" src="github-brands.svg" />
          <a href="https://github.com/chriamue/ornithology-pi">{"chriamue/ornithology-pi"}</a>
        </p>
      </div>
    </footer>
    }
}
