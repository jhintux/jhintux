use dioxus::prelude::*;

#[component]
pub fn Loading() -> Element {
    rsx! {
        div {
            class: "flex items-center justify-center h-full",
            "Loading..."
        }
    }
}