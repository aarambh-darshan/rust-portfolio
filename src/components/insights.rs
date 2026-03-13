// src/components/insights.rs — Darshan Vichhi Portfolio × Leptos 0.7
//
// ── UI COMPONENT ─────────────────────────────────────────────────────────────
// Fully encapsulated component. Data is injected statically at compile time.
// ─────────────────────────────────────────────────────────────────────────────

use leptos::prelude::*;
use crate::content;

#[component]
pub fn Insights() -> impl IntoView {
    let articles = content::load().articles;
    view! {
        <section class="insights-section">
            <div class="grid grid-cols-12 border-b">
                <div class="col-span-12 p-[clamp(16px,2.5vw,32px)] flex flex-col justify-center border-b">
                    <h2 class="section-heading font-sans text-[0.7rem] font-semibold tracking-[0.25em] uppercase text-muted">"INSIGHTS"</h2>
                </div>
            </div>
            {articles.into_iter().map(|a| view! {
                <a href=a.href target="_blank"
                   class="article-row grid grid-cols-[auto_1fr_auto] md:grid-cols-12 border-b hover-target">
                    <div class="col-span-1 md:col-span-2 p-[clamp(16px,2.5vw,32px)] border-r text-[0.6rem] font-semibold tracking-[0.2em] text-red flex items-center">{a.kind}</div>
                    <div class="col-span-1 md:col-span-9 p-[clamp(16px,2.5vw,32px)] border-r font-serif text-[clamp(0.9rem,1.5vw,1.3rem)] font-bold flex items-center">{a.title}</div>
                    <div class="col-span-1 md:col-span-1 p-[clamp(16px,2.5vw,32px)] text-[1.2rem] flex items-center justify-center art-arrow">"↗"</div>
                </a>
            }).collect_view()}
        </section>
    }
}