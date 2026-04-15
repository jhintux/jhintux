use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "px-8 py-6 flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4",
            span { class: "text-sm text-zinc-400 dark:text-zinc-500", "jhintux.dev · 2026" }
            div { class: "flex flex-col sm:flex-row sm:items-center gap-3",
                span { class: "text-xs text-zinc-400 dark:text-zinc-500",
                    "projects & TILs powered by Contentful"
                }
                span { class: "text-xs px-2 py-1 bg-orange-50 text-orange-700 border border-orange-100 rounded-md font-medium w-fit dark:bg-orange-500/15 dark:text-orange-300 dark:border-orange-500/30",
                    "built with Dioxus"
                }
            }
        }
    }
}
