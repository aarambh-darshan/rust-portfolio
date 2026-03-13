// src/components/hero.rs — Darshan Vichhi Portfolio × Leptos 0.7
//
// ── UI COMPONENT ─────────────────────────────────────────────────────────────
// Fully encapsulated component. Data is injected statically at compile time.
// ─────────────────────────────────────────────────────────────────────────────

use leptos::prelude::*;
use crate::content;

#[component]
pub fn Hero() -> impl IntoView {
    let c = content::load();
    let h = c.hero;
    let stats = h.stats.clone();
    view! {
        <section class="hero-section grid grid-cols-1 md:grid-cols-12 border-b">
            <div class="hidden md:flex md:col-span-2 p-[clamp(16px,2.5vw,32px)] flex-col justify-center border-r">
                <p class="font-sans text-[0.6rem] font-medium tracking-[0.2em] uppercase text-muted [writing-mode:vertical-rl] [text-orientation:mixed]">{c.meta.edition}</p>
            </div>
            <div class="col-span-full md:col-span-7 p-[clamp(32px,5vw,80px)] flex flex-col justify-center border-r">
                <h1 class="hero-title font-serif text-[clamp(3rem,8vw,7.5rem)] font-black leading-[0.95] tracking-[-0.03em]">
                    <span class="line">{h.headline_1}</span>
                    <span class="line italic">{h.headline_2}</span>
                </h1>
                <p class="hero-body text-[clamp(0.85rem,1.1vw,1rem)] leading-[1.7] text-muted max-w-[460px] mt-7">{h.body}</p>
            </div>
            <div class="col-span-full md:col-span-3 flex flex-row md:flex-col p-[clamp(32px,5vw,80px)] md:p-0">
                {stats.into_iter().map(|s| {
                    view! {
                        <div class="stat-block flex-1 flex flex-col justify-center p-[clamp(16px,2.5vw,32px)] gap-1 border-r last:border-r-0 md:border-r-0 md:border-b md:last:border-b-0">
                            <span class="font-serif text-[clamp(1.8rem,3vw,2.8rem)] font-black text-red">{s.num}</span>
                            <span class="text-[0.6rem] tracking-[0.2em] text-muted">{s.label}</span>
                        </div>
                    }
                }).collect_view()}
            </div>
        </section>
    }
}