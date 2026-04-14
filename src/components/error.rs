use dioxus::{logger, prelude::*};

#[component]
pub fn Error(error: String) -> Element {
logger::tracing::error!("Error loading content: {}", error);
    
    rsx! {
        div {
            class: "flex items-center justify-center h-full",
            "Error loading content"
        }
    }
}