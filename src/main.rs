#![allow(non_snake_case, unused)]

use dioxus::prelude::*;
use components::header::Header;

mod components {
    pub mod header;
}

fn main() {
    launch(app)
}

fn app() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
        Header{},
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
    }
}

