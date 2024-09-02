#![allow(non_snake_case)]
mod nav_bar;
mod pages;
mod router;

use crate::router::Route;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

pub fn App() -> Element {
    rsx! {
      div{ class: "min-h-screen bg-main-color",
        Router::<Route> {}
      }
    }
}
