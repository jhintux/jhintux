use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct PortfolioTagProps {
    label: String,
    #[props(default)]
    class: String,
}

#[component]
pub fn PortfolioTag(props: PortfolioTagProps) -> Element {
    rsx! {
        span {
            class: "text-xs px-2 py-0.5 rounded border font-normal {props.class}",
            "{props.label}"
        }
    }
}
