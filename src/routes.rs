#![allow(non_snake_case, unused)]

use crate::views::PageNotFound::Page as PageNotFound;
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
