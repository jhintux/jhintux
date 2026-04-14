use crate::components::{Footer, PortfolioNav};
use crate::route::Route;
use dioxus::prelude::*;

#[component]
pub fn Main() -> Element {
    rsx! {
        div { class: "min-h-screen bg-white font-sans text-base antialiased text-zinc-900",
            div { class: "mx-4",
                PortfolioNav {}
                Outlet::<Route> {}
                Footer {}
            }
        }
    }
}
