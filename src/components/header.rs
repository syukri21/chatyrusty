use dioxus::prelude::{Element, rsx};

pub(crate) fn Header() -> Element {
    rsx! {
        head {
            link { rel: "stylesheet", href: "/public/tailwindl.css" }
        }
    }
}