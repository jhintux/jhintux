use crate::components::{Loading, Error, SectionLabel};
use crate::utils::scroll_to_id;
use crate::contentful::{fetch_projects, fetch_tils};
use crate::views::{ExperienceSection, ProjectsSection, TilsSection};
use dioxus::prelude::*;

const TECH_TAGS: &[(&str, &str)] = &[
    ("Axum", "bg-orange-50 text-orange-800 border-orange-200"),
    ("Springboot", "bg-blue-50 text-blue-800 border-blue-200"),
    ("Next.js", "bg-zinc-100 text-zinc-600 border-zinc-200"),
    ("Dioxus", "bg-sky-100 text-sky-600 border-sky-200"),
    ("PostgreSQL", "bg-green-100 text-green-600 border-green-200"),
];

const CONTACT: &[(&str, &str, &str)] = &[
    ("GitHub", "github.com/jhintux", "https://github.com/jhintux"),
    (
        "LinkedIn",
        "linkedin.com/in/eduardolanasca",
        "https://www.linkedin.com/in/eduardolanasca",
    ),
    ("Email", "edujlac@gmail.com", "mailto:edujlac@gmail.com"),
];

#[component]
fn HeroSection() -> Element {
    rsx! {
        section {
            id: "about",
            class: "px-8 pt-16 pb-14 border-b border-zinc-100",
            p { class: "text-xs tracking-widest uppercase text-zinc-400 mb-3", "Fullstack engineer" }
            h1 { class: "text-5xl font-semibold tracking-tight text-zinc-900 leading-tight mb-2", "Jhintux" }
            p { class: "text-base text-zinc-500 leading-relaxed mb-1 max-w-xl",
                "Building backend systems in Rust & Java with a crypto background."
                br {}
                "Frontend in React / Next.js — and pushing into Dioxus (Rust → WASM)."
            }
            p { class: "text-base text-zinc-400 leading-relaxed mb-6 max-w-xl",
                "Based in Medellín, Colombia. Open to remote opportunities."
            }
            div { class: "flex flex-wrap gap-2 mb-8",
                for (label, color) in TECH_TAGS.iter().copied() {
                    span {
                        key: "{label}",
                        class: "text-sm px-3 py-1 rounded-md border font-normal {color}",
                        "{label}"
                    }
                }
            }
            div { class: "flex items-center gap-3 flex-wrap",
                button {
                    class: "text-sm px-5 py-2 bg-zinc-900 text-white rounded-lg hover:bg-zinc-700 transition-colors duration-150 font-medium",
                    onclick: move |_| scroll_to_id("projects"),
                    "View projects"
                }
                a {
                    class: "text-sm px-5 py-2 border border-zinc-200 text-zinc-600 rounded-lg hover:bg-zinc-50 hover:border-zinc-300 transition-all duration-150",
                    href: "https://drive.google.com/file/d/1gyg_F7ty8_4jwfvzIhscb366cJJBSub-/view?usp=sharing",
                    download: true,
                    "Download CV"
                }
                a {
                    class: "text-sm px-5 py-2 border border-zinc-200 text-zinc-600 rounded-lg hover:bg-zinc-50 hover:border-zinc-300 transition-all duration-150",
                    href: "https://github.com/jhintux",
                    target: "_blank",
                    rel: "noreferrer",
                    "GitHub"
                }
            }
        }
    }
}

#[component]
fn ContactSection() -> Element {
    rsx! {
        section {
            id: "contact",
            class: "px-8 py-10 border-b border-zinc-100",
            SectionLabel { cms: false, "Contact" }
            div { class: "grid grid-cols-1 sm:grid-cols-3 gap-3 mb-8",
                for (label, value, href) in CONTACT.iter().copied() {
                    a {
                        key: "{label}",
                        class: "group block p-4 border border-zinc-100 rounded-xl hover:border-zinc-200 hover:bg-zinc-50/50 transition-all duration-150",
                        href,
                        target: if href.starts_with("mailto:") { None } else { Some("_blank") },
                        rel: "noreferrer",
                        p { class: "text-xs uppercase tracking-widest text-zinc-400 mb-1", "{label}" }
                        p { class: "text-sm text-blue-600 group-hover:text-blue-700 transition-colors duration-150 break-all",
                            "{value}"
                        }
                    }
                }
            }
            div { class: "flex items-center gap-3 p-4 bg-emerald-50 border border-emerald-100 rounded-xl",
                span { class: "w-2 h-2 rounded-full bg-emerald-500 shrink-0 animate-pulse" }
                p { class: "text-sm text-emerald-700",
                    "Open to remote fullstack / backend roles. Particularly interested in Rust, systems programming, or fintech."
                }
            }
        }
    }
}

/// Full portfolio landing page: hero, projects, experience, TILs, and contact (matches `Portfolio.jsx`).
#[component]
pub fn Home() -> Element {
    let projects = use_resource(|| fetch_projects(None, None));
    let tils = use_resource(|| fetch_tils(None, None));

    rsx! {
        main {
            HeroSection {}
            match &*projects.read() {
                Some(Ok(projects)) => rsx! { ProjectsSection { projects: projects.clone() } },
                Some(Err(e)) => rsx! { Error { error: e } },
                None => rsx! { Loading {} },
            }
            ExperienceSection {}
            match &*tils.read() {
                Some(Ok(tils)) => rsx! { TilsSection { tils: tils.clone() } },
                Some(Err(e)) => rsx! { Error { error: e.to_string() } },
                None => rsx! { Loading {} },
            }
            ContactSection {}
        }
    }
}
