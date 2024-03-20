#![allow(non_snake_case, unused)]

use components::header::Header;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod components {
    pub mod header;
}

mod views {
    pub mod SignIn;
    pub mod SignUp;
}

mod routes;

fn main() {
    launch(App)
}

pub fn App() -> Element {
    rsx! { Router::<routes::Routes> {} }
}
