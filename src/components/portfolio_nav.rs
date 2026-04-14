use crate::route::Route;
use crate::utils::scroll_to_id;
use dioxus::prelude::*;

const NAV_LINKS: &[&str] = &["about", "projects", "experience", "TILs"];

#[component]
pub fn PortfolioNav() -> Element {
    let nav = use_navigator();
    let route = use_route::<Route>();
    let mut home_active = use_signal(|| "about".to_string());
    let mut mobile_menu_open = use_signal(|| false);

    let highlight = match route {
        Route::Home {} => home_active(),
        Route::Projects {} => "projects".to_string(),
        Route::Experience {} => "experience".to_string(),
        Route::Tils {} => "TILs".to_string(),
    };

    rsx! {
        div {
            class: "sticky top-0 z-50 bg-white/95 backdrop-blur-sm border-b border-zinc-100",
            nav {
                class: "flex items-center justify-between px-4 py-4 sm:px-8",
                span { class: "text-base font-semibold tracking-tight text-zinc-900 select-none",
                    "JH."
                }
                div { class: "hidden md:flex items-center gap-8",
                    for link in NAV_LINKS.iter().copied() {
                        button {
                            class: if highlight == link {
                                "text-sm transition-colors duration-150 text-zinc-900 font-medium"
                            } else {
                                "text-sm transition-colors duration-150 text-zinc-400 hover:text-zinc-700"
                            },
                            onclick: {
                                let link = link.to_string();
                                move |_| {
                                    *home_active.write() = link.clone();
                                    nav.push(Route::Home {});
                                    let id = link.clone();
                                    scroll_to_id(&id);
                                }
                            },
                            "{link}"
                        }
                    }
                }
                div { class: "flex items-center gap-2 sm:gap-3",
                    button {
                        class: "text-sm px-3 py-1.5 sm:px-4 border border-zinc-200 rounded-lg text-zinc-700 hover:bg-zinc-50 hover:border-zinc-300 transition-all duration-150",
                        onclick: move |_| {
                            *home_active.write() = "contact".to_string();
                            nav.push(Route::Home {});
                            scroll_to_id("contact");
                            *mobile_menu_open.write() = false;
                        },
                        "contact"
                    }
                    button {
                        class: "md:hidden inline-flex items-center justify-center w-9 h-9 rounded-lg border border-zinc-200 text-zinc-700 hover:bg-zinc-50 hover:border-zinc-300 transition-all duration-150",
                        "aria-label": "Open navigation menu",
                        onclick: move |_| *mobile_menu_open.write() = !mobile_menu_open(),
                        svg {
                            width: "18",
                            height: "18",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            line { x1: "3", y1: "6", x2: "21", y2: "6" }
                            line { x1: "3", y1: "12", x2: "21", y2: "12" }
                            line { x1: "3", y1: "18", x2: "21", y2: "18" }
                        }
                    }
                }
            }
            if mobile_menu_open() {
                div { class: "md:hidden border-t border-zinc-100 px-4 pb-4 pt-3",
                    div { class: "flex flex-col gap-2",
                        for link in NAV_LINKS.iter().copied() {
                            button {
                                class: if highlight == link {
                                    "text-left text-zinc-900 font-medium px-3 py-2 rounded-lg bg-zinc-100"
                                } else {
                                    "text-left text-zinc-500 px-3 py-2 rounded-lg hover:bg-zinc-50 hover:text-zinc-700"
                                },
                                onclick: {
                                    let link = link.to_string();
                                    move |_| {
                                        *home_active.write() = link.clone();
                                        nav.push(Route::Home {});
                                        let id = link.clone();
                                        scroll_to_id(&id);
                                        *mobile_menu_open.write() = false;
                                    }
                                },
                                "{link}"
                            }
                        }
                    }
                }
            }
        }
    }
}
