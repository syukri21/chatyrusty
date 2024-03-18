use dioxus::prelude::*;
use dioxus::dioxus_core;

pub(crate) fn Header() -> Element {
    const TAILWIND: &str = manganis::mg!(file("./public/tailwind.css"));
    rsx! {
        head {
            link { rel: "stylesheet", href: "{TAILWIND}" }
        }
    }
}