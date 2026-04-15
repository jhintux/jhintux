use crate::components::{Footer, PortfolioNav};
use crate::route::Route;
use dioxus::prelude::*;

#[component]
pub fn Main() -> Element {
    rsx! {
        div { class: "min-h-screen bg-white font-sans text-base antialiased text-zinc-900 dark:bg-zinc-950 dark:text-zinc-100",
            div { class: "mx-4 min-h-screen flex flex-col",
                PortfolioNav {}
                main { class: "flex-1",
                    Outlet::<Route> {}
                }
                Footer {}
            }
        }
    }
}
