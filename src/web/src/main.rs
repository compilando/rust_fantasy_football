use yew::prelude::*;

mod color_utils;
mod components;
mod settings;
mod engine;

use components::game::Game;
use settings::{default_settings, Settings};

#[function_component(App)]
fn app() -> Html {
  html! {
    <ContextProvider<Settings> context={default_settings()}>
      <Game/>
    </ContextProvider<Settings>>
  }
}

fn main() {
  yew::start_app::<App>();
}