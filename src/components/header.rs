use dioxus::prelude::*;
use dioxus::dioxus_core;

pub(crate) fn Header() -> Element {
    const _TAILWIND_URL: &str = manganis::mg!(file("./public/tailwind.css"));
    rsx! {
        head {
        }
    }
}