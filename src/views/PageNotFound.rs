#![allow(non_snake_case, unused)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub(crate) fn Page(route: Vec<String>) -> Element {
    rsx! {
        div {
            class:"flex min-h-full flex-col justify-center px-6 py-12 lg:px-8",
            div {
                class:"p-5",
                 p{
                    class: "mb-2 text-2xl font-bold tracking-tight text-gray-500 ",
                    "Your lost?"
                },
                p{
                    class: "mb-2 text-xl font-bold tracking-tight text-gray-900",
                    "get back to an awesome experience! "
                }
                a {
                    href: "/",
                    class: "my-6  inline-flex items-center justify-center px-3 py-1 border border-transparent text-base font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700",
                    "Go Home"
                }
            }
        }

    }
}
