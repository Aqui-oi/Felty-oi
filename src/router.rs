use dioxus::prelude::*;
use crate::components::{
    about::About,
    home::Home,
};

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum AppRoute {
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
}