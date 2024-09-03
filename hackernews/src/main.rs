#![allow(non_snake_case)]

use dioxus::prelude::*;
use hackernews::App;
use tracing::{info, Level};

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting hackernews app");
    launch(App);
}
