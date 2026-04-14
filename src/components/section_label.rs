use dioxus::prelude::*;

#[component]
fn CmsBadge() -> Element {
    rsx! {
        span {
            class: "inline-flex items-center gap-1.5 text-xs px-2 py-1 bg-zinc-50 border border-zinc-200 rounded-md text-zinc-400 font-normal float-right",
            span { class: "w-1.5 h-1.5 rounded-full bg-emerald-500 inline-block" }
            "CMS managed"
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct SectionLabelProps {
    children: Element,
    #[props(default = false)]
    cms: bool,
}

#[component]
pub fn SectionLabel(props: SectionLabelProps) -> Element {
    rsx! {
        div { class: "flex items-center justify-between mb-6",
            span { class: "text-xs tracking-widest uppercase text-zinc-400 font-medium",
                {props.children}
            }
            if props.cms {
                CmsBadge {}
            }
        }
    }
}
