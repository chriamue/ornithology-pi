use yew::prelude::*;
use yew_bootstrap::component::{Container, ContainerSize};

#[function_component(About)]
pub fn comp() -> Html {
    html! {
      <div class="about">
        <Container class="bg-info" size={ContainerSize::ExtraLarge}>
          <p>{"
            The goal of this project is a raspberry pi device with a camera, that
            films your garden. If it detects a bird, it takes a picture and identifies
            the species of this bird.
            "}
          </p>
          <p>{"
            The device should be connactable via bluetooth or wifi to view the taken
            pictures.
            "}
          </p>
        </Container>
      </div>
    }
}
