use dioxus::prelude::*;
use crate::components::{PortfolioTag, SectionLabel};
use crate::contentful::Project;


#[component]
fn ProjectCard(project: Project) -> Element {
    let mut hovered = use_signal(|| false);

    let border = if project.featured {
        "relative bg-white rounded-xl p-5 transition-all duration-200 flex flex-col gap-3 border-l-2 border-l-orange-400 border-t border-r border-b border-zinc-100"
    } else {
        "relative bg-white rounded-xl p-5 transition-all duration-200 flex flex-col gap-3 border border-zinc-100 hover:border-zinc-200"
    };
    let shadow = if hovered() { "shadow-sm" } else { "" };

    rsx! {
        div {
            class: "{border} {shadow}",
            onmouseenter: move |_| *hovered.write() = true,
            onmouseleave: move |_| *hovered.write() = false,
            if let Some(ref badge) = project.badge {
                span {
                    class: "absolute top-4 right-4 text-xs px-2 py-0.5 bg-orange-50 text-orange-700 border border-orange-200 rounded-md",
                    "{badge}"
                }
            }
            div {
                h3 { class: "text-base font-medium text-zinc-900 mb-1.5 pr-20", "{project.title}" }
                p { class: "text-sm text-zinc-500 leading-relaxed", "{project.description}" }
            }
            div { class: "flex flex-wrap gap-1.5 mt-auto",
                for tag in project.tags.iter() {
                    PortfolioTag {
                        label: tag.clone(),
                        class: "bg-zinc-50 text-zinc-500 border-zinc-100".to_string(),
                    }
                }
            }
            div { class: "flex items-center gap-3 pt-1 border-t border-zinc-50",
                if let Some(ref url) = project.github {
                    a {
                        class: "text-sm text-zinc-400 hover:text-zinc-700 transition-colors duration-150 flex items-center gap-1",
                        href: "{url}",
                        target: "_blank",
                        rel: "noreferrer",
                        svg {
                            width: "12",
                            height: "12",
                            view_box: "0 0 24 24",
                            fill: "currentColor",
                            path { d: "M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0 0 24 12c0-6.63-5.37-12-12-12z" }
                        }
                        "source"
                    }
                }
                if let Some(ref url) = project.live {
                    a {
                        class: "text-sm text-zinc-400 hover:text-zinc-700 transition-colors duration-150 flex items-center gap-1",
                        href: "{url}",
                        target: "_blank",
                        rel: "noreferrer",
                        svg {
                            width: "11",
                            height: "11",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" }
                            polyline { points: "15 3 21 3 21 9" }
                            line { x1: "10", y1: "14", x2: "21", y2: "3" }
                        }
                        "live"
                    }
                }
            }
        }
    }
}

#[component]
pub fn ProjectsSection(projects: Vec<Project>) -> Element {
    rsx! {
        section {
            id: "projects",
            class: "px-8 py-10 border-b border-zinc-100",
            SectionLabel { cms: true, "Projects" }
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                for p in projects {
                    ProjectCard {
                        key: "{p.id}",
                        project: p,
                    }
                }
            }
        }
    }
}

#[component]
pub fn Projects() -> Element {
    
    rsx! {

    }
}
