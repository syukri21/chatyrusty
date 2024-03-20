#![allow(non_snake_case, unused)]

use crate::views::SignIn::Page as SignIn;
use crate::views::SignUp::Page as SignUp;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, PartialEq, Clone)]
pub enum Routes {
    #[route("/signup")]
    SignUp {},
    #[route("/signin")]
    SignIn {},
    //  if the current location doesn't match any of the above routes, render the NotFound component
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}
