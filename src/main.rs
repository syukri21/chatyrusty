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
    rsx! {
        Header{},
        div {
            class: "flex min-h-full flex-col justify-center px-6 py-12 lg:px-8",
            div {
                class: "sm:mx-auto sm:w-full sm:max-w-sm",
                h2 {
                    class: "mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-gray-900",
                    "Hello, world!"
                },
                p {
                    class: "mt-10 text-center text-sm text-gray-600",
                    "Don't have an account? ",
                    a {
                        class: "font-semibold leading-6 text-indigo-600 hover:text-indigo-500",
                        href: "#",
                        "Sign up"
                    }
                },
                p {
                    class: "mt-10 text-center text-sm text-gray-600",
                    "Already have an account? ",
                    a {
                        class: "font-semibold leading-6 text-indigo-600 hover:text-indigo-500",
                        href: "#",
                        "Sign in"
                    }
                },
                p {
                    class: "mt-10 text-center text-sm text-gray-600",
                    "By continuing, you agree to our ",
                    a {
                        class: "font-semibold leading-6 text-indigo-600 hover:text-indigo-500",
                        href: "#",
                        "terms and conditions"
                    }
                }
            }
        }
    }
}

