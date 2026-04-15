use crate::components::{Error, Loading, PortfolioTag, SectionLabel};
use crate::contentful::{fetch_til, Til};
use crate::route::Route;
use dioxus::prelude::*;

fn tag_class(tag: &str) -> String {
    match tag {
        "Rust" => "bg-orange-50 text-orange-700 border-orange-100 dark:bg-orange-500/15 dark:text-orange-300 dark:border-orange-500/30",
        "Dioxus" => "bg-blue-50 text-blue-700 border-blue-100 dark:bg-blue-500/15 dark:text-blue-300 dark:border-blue-500/30",
        "Architecture" => "bg-violet-50 text-violet-700 border-violet-100 dark:bg-violet-500/15 dark:text-violet-300 dark:border-violet-500/30",
        "PostgreSQL" => "bg-emerald-50 text-emerald-700 border-emerald-100 dark:bg-emerald-500/15 dark:text-emerald-300 dark:border-emerald-500/30",
        _ => "bg-zinc-50 text-zinc-500 border-zinc-100 dark:bg-zinc-800 dark:text-zinc-300 dark:border-zinc-700",
    }
    .to_string()
}

fn format_til_date(date: &str) -> String {
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() < 2 {
        return date.to_string();
    }

    let month = match parts[1] {
        "01" => "Jan",
        "02" => "Feb",
        "03" => "Mar",
        "04" => "Apr",
        "05" => "May",
        "06" => "Jun",
        "07" => "Jul",
        "08" => "Aug",
        "09" => "Sep",
        "10" => "Oct",
        "11" => "Nov",
        "12" => "Dec",
        _ => return date.to_string(),
    };

    format!("{month} {}", parts[0])
}

#[component]
fn TilRow(til: Til) -> Element {
    let nav = use_navigator();
    let til_id = til.id.clone();
    let formatted_date = format_til_date(&til.date);
    rsx! {
        div {
            class: "flex flex-col sm:flex-row sm:items-center sm:justify-between gap-2 py-3 px-4 border border-zinc-100 rounded-xl hover:border-zinc-200 hover:bg-zinc-50/50 transition-all duration-150 cursor-pointer group dark:border-zinc-800 dark:hover:border-zinc-700 dark:hover:bg-zinc-900/60",
            onclick: move |_| {
                nav.push(Route::TilDetail { id: til_id.clone() });
            },
            div { class: "flex items-center gap-3 min-w-0",
                svg {
                    width: "12",
                    height: "12",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    class: "text-zinc-300 group-hover:text-zinc-500 shrink-0 transition-colors duration-150 dark:text-zinc-600 dark:group-hover:text-zinc-300",
                    line { x1: "5", y1: "12", x2: "19", y2: "12" }
                    polyline { points: "12 5 19 12 12 19" }
                }
                span { class: "text-sm text-zinc-700 break-words whitespace-normal leading-6 dark:text-zinc-200", "{til.title}" }
            }
            div { class: "flex flex-wrap items-center gap-2 sm:gap-3 sm:shrink-0 sm:ml-4",
                for tag in til.tags.iter() {
                    PortfolioTag {
                        key: "{tag}",
                        label: tag.to_string(),
                        class: tag_class(tag.as_str()),
                    }
                }
                span { class: "text-sm text-zinc-400 whitespace-nowrap dark:text-zinc-500", "{formatted_date}" }
            }
        }
    }
}

#[component]
pub fn TilsSection(tils: Vec<Til>) -> Element {
    rsx! {
        section {
            id: "TILs",
            class: "px-8 py-10 border-b border-zinc-100 dark:border-zinc-800",
            SectionLabel { cms: true, "TILs — today I learned" }
            div { class: "flex flex-col gap-2",
                for til in tils {
                    TilRow {
                        key: "{til.id}",
                        til,
                    }
                }
            }
            p { class: "text-sm text-zinc-400 mt-4 dark:text-zinc-500",
                "Short notes on things I pick up while learning Rust, Java, and systems programming."
            }
        }
    }
}

#[component]
pub fn TilDetail(id: String) -> Element {
    let til_resource = use_resource({
        let id = id.clone();
        move || fetch_til(id.clone())
    });

    rsx! {
        main { class: "px-8 py-8",
            match &*til_resource.read() {
                Some(Ok(til)) => rsx! {
                    section { class: "max-w-5xl mx-auto flex flex-col gap-6",
                        div { class: "flex flex-col gap-4 md:flex-row md:items-center md:justify-between",
                            div { class: "flex flex-col items-start gap-2 md:flex-row md:items-end md:gap-4",
                                h1 { class: "text-4xl font-semibold tracking-tight text-zinc-900 dark:text-zinc-100",
                                    "{til.title}"
                                }
                                span { class: "text-base text-zinc-500 dark:text-zinc-400 md:pb-1",
                                    "{format_til_date(&til.date)}"
                                }
                            }

                            div { class: "flex flex-wrap items-center gap-2",
                                for tag in til.tags.iter() {
                                    PortfolioTag {
                                        key: "{tag}",
                                        label: tag.to_string(),
                                        class: tag_class(tag.as_str()),
                                    }
                                }
                            }
                        }

                        article { class: "min-h-[520px] w-full p-4 sm:p-6 md:p-8 text-base leading-7 md:leading-8 text-zinc-700 whitespace-pre-wrap dark:text-zinc-200",
                            "{til.article}"
                        }
                    }
                },
                Some(Err(e)) => rsx! { Error { error: e.to_string() } },
                None => rsx! { Loading {} },
            }
        }
    }
}

#[component]
pub fn Tils() -> Element {
    rsx! {
        
    }
}
