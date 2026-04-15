use crate::components::{PortfolioTag, SectionLabel};
use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub struct ExperienceEntry {
    pub id: i32,
    pub period: String,
    pub role: String,
    pub company: String,
    pub location: String,
    pub description: String,
    pub tags: Vec<String>,
}

fn experience() -> Vec<ExperienceEntry> {
    vec![
        ExperienceEntry {
            id: 1,
            period: "04/2024 - 09/2025".into(),
            role: "Rust Software Engineer (Full-Stack, Blockchain)".into(),
            company: "Quantum3Labs".into(),
            location: "Remote - Asia".into(),
            description: "Built and open-sourced a full Scaffold-ETH adaptation for Arbitrum Stylus, adopted by hundreds of external developers and reused as a base template across multiple projects".into(),
            tags: vec!["React".into(), "Next.js".into(), "Node.js".into(), "Web3.js".into(), "Rust".into()],
        },
        ExperienceEntry {
            id: 2,
            period: "05/2023 – 03/2024".into(),
            role: "Software Engineer (Blockchain, Full-Stack)".into(),
            company: "Smithii".into(),
            location: "Remote".into(),
            description: "Built data ingestion and normalization services across 9 APIs (Payments, Solana Market Data, Mail, Discord and X), supporting both real-time and batch pipelines with improved reliability under growth.".into(),
            tags: vec!["React".into(), "Next.js".into(), "Springboot".into(), "Web3.js".into(), "Rust".into()],
        },
        ExperienceEntry {
            id: 3,
            period: "04/2023 – 09/2023".into(),
            role: "Rust Software Engineer".into(),
            company: "Foxchain".into(),
            location: "Remote - Asia".into(),
            description: "Ported the FOAKS algorithm from C++ to Rust, cutting execution time from ~30s to ~12s while improving memory efficiency".into(),
            tags: vec!["Rust".into(), "C++".into()],
        },
        ExperienceEntry {
            id: 4,
            period: "05/2022 – 03/2023".into(),
            role: "Founder & Full Stack Engineer".into(),
            company: "The Castle".into(),
            location: "Remote".into(),
            description: "Built and owned the entire decentralized platform —from smart contracts to frontend— enabling DAOs to trade whitelist access and generating $10,000+ in early revenue through automated fee collection.".into(),
            tags: vec!["React".into(), "Next.js".into(), "Node.js".into(), "Web3.js".into(), "Rust".into()],
        },
        ExperienceEntry {
            id: 5,
            period: "04/2022 – 11/2022".into(),
            role: "Full stack analyst".into(),
            company: "Odybank".into(),
            location: "Lima, PE".into(),
            description: "Developed and deployed a batch-processing system that increased document handling capacity from 250 to 1,000 files and reduced processing time from one week to just three days.".into(),
            tags: vec!["Springboot".into(), "MySQL".into()],
        },
        ExperienceEntry {
            id: 6,
            period: "12/2020 - 11/2021".into(),
            role: "TI Division - Trainee".into(),
            company: "Banco de Credito del Peru".into(),
            location: "Lima, PE".into(),
            description: "Cut process time from 45 to 3 minutes by standardizing formats and automating recurring data request emails; built a script to detect missing data improving allocation of $2M in data mart costs; developed a PowerApps solution used by 180+ bank employees; and analyzed IT consumption data to identify cost-saving opportunities.".into(),
            tags: vec!["PowerApps".into(), "Python".into(), "SQL".into()],
        }
    ]
}

#[component]
fn ExperienceItem(entry: ExperienceEntry, is_last: bool) -> Element {
    let border = if !is_last {
        "flex gap-4 sm:gap-6 py-5 border-b border-zinc-100 dark:border-zinc-800"
    } else {
        "flex gap-4 sm:gap-6 py-5"
    };

    rsx! {
        div {
            class: "{border}",
            div { class: "flex flex-col items-center pt-[6px] min-w-[16px]",
                div { class: "w-2 h-2 rounded-full bg-zinc-300 shrink-0 dark:bg-zinc-600" }
                if !is_last {
                    div { class: "w-px flex-1 bg-zinc-100 mt-2 dark:bg-zinc-800" }
                }
            }
            div { class: "flex-1",
                p { class: "text-sm text-zinc-400 mb-1.5 dark:text-zinc-500", "{entry.period}" }
                div { class: "flex items-baseline gap-2 mb-0.5",
                    h3 { class: "text-base font-medium text-zinc-900 dark:text-zinc-100", "{entry.role}" }
                }
                p { class: "text-sm text-zinc-500 mb-2 dark:text-zinc-400", "{entry.company} · {entry.location}" }
                p { class: "text-sm text-zinc-400 leading-relaxed mb-3 dark:text-zinc-500", "{entry.description}" }
                div { class: "flex flex-wrap gap-1.5",
                    for tag in entry.tags.iter() {
                        PortfolioTag {
                            label: tag.clone(),
                            class: "bg-zinc-50 text-zinc-400 border-zinc-100 dark:bg-zinc-800 dark:text-zinc-300 dark:border-zinc-700".to_string(),
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ExperienceSection() -> Element {
    let entries = experience();
    let n = entries.len();

    rsx! {
        section {
            id: "experience",
            class: "px-8 py-10 border-b border-zinc-100 dark:border-zinc-800",
            SectionLabel { cms: false, "Experience" }
            div {
                for (i, item) in entries.into_iter().enumerate() {
                    ExperienceItem {
                        key: "{item.id}",
                        entry: item,
                        is_last: i + 1 == n,
                    }
                }
            }
        }
    }
}

#[component]
pub fn Experience() -> Element {
    rsx! {
        ExperienceSection {}
    }
}
