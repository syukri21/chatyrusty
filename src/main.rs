#![allow(non_snake_case, unused)]

use dioxus::prelude::*;
use dioxus_logger::DioxusLogger;
use dioxus_router::prelude::*;
use log::LevelFilter;

mod views {
    pub mod PageNotFound;
    pub mod SignIn;
    pub mod SignUp;
}
mod routes;

fn main() {
    DioxusLogger::new(LevelFilter::Info)
        .use_format("[{LEVEL}] {PATH} - {ARGS}")
        .build()
        .expect("failed to init logger");
    launch(App)
}

pub fn App() -> Element {
    rsx! { Router::<routes::Routes> {} }
}
