use dioxus::prelude::*;
use crate::views::*;

/// The Route enum is used to define the structure of internal routes in our app. All route enums need to derive
/// the [`Routable`] trait, which provides the necessary methods for the router to work.
/// 
/// Each variant represents a different URL pattern that can be matched by the router. If that pattern is matched,
/// the components for that route will be rendered.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // The layout attribute defines a wrapper for all routes under the layout. Layouts are great for wrapping
    // many routes with a common UI like a navbar.
    #[layout(Main)]
    // The route attribute defines the URL pattern that a specific route matches. If that pattern matches the URL,
    // the component for that route will be rendered. The component name that is rendered defaults to the variant name.
    #[route("/")]
    Home {},
    #[route("/projects")]
    Projects {},
    #[route("/experience")]
    Experience {},
    #[route("/tils")]
    Tils {},
}