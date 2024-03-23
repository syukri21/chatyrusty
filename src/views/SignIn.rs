#![allow(non_snake_case, unused)]

use dioxus::dioxus_core;
use dioxus::prelude::*;

pub(crate) fn Page() -> Element {
    rsx! {
        div { class: "flex min-h-full flex-col justify-center px-6 py-12",
        div { class: "sm:mx-auto sm:w-full sm:max-w-sm",
        img {
                    alt: "Your Company",
                    src: "https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600",
                    class: "mx-auto h-10 w-auto"
                }
                h2 { class: "mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-gray-900",
                    "Welcome back!" div {"Sign in to your account"}
                }
            }
            div { class: "mt-10 sm:mx-auto sm:w-full sm:max-w-sm",
                form { action: "#", method: "POST", class: "space-y-6",
                    div {
                        label {
                            r#for: "email",
                            class: "block text-sm font-medium leading-6 text-gray-900",
                            "Email address"
                        }
                        div { class: "mt-2",
                            input {
                                required: "false",
                                name: "email",
                                r#type: "email",
                                autocomplete: "email",
                                class: "block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6",
                                id: "email"
                            }
                        }
                    }
                    div {
                        div { class: "flex items-center justify-between",
                            label {
                                r#for: "password",
                                class: "block text-sm font-medium leading-6 text-gray-900",
                                "Password"
                            }
                            div { class: "text-sm",
                                a {
                                    href: "#",
                                    class: "font-semibold text-indigo-600 hover:text-indigo-500",
                                    "Forgot password?"
                                }
                            }
                        }
                        div { class: "mt-2",
                            input {
                                autocomplete: "current-password",
                                required: "false",
                                name: "password",
                                r#type: "password",
                                class: "block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6",
                                id: "password"
                            }
                        }
                    }
                    div {
                        button {
                            r#type: "submit",
                            class: "flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
                            "Sign in"
                        }
                    }
                }
                p { class: "mt-10 text-center text-sm text-gray-500",
                    "Not a member? "
                    a {
                        href: "/signup",
                        class: "font-semibold leading-6 text-indigo-600 hover:text-indigo-500",
                        "Sign up now!"
                    }
                }
            }
        }
    }
}
