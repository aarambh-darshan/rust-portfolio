// src/components/ticker.rs — Darshan Vichhi Portfolio × Leptos 0.7
//
// ── UI COMPONENT ─────────────────────────────────────────────────────────────
// Fully encapsulated component. Data is injected statically at compile time.
// ─────────────────────────────────────────────────────────────────────────────

use leptos::prelude::*;
use crate::content;

#[component]
pub fn Ticker(
    #[prop(default = false)] reverse: bool,
) -> impl IntoView {
    let track_class = if reverse { "ticker-track reverse" } else { "ticker-track" };
    
    let meta = content::load().meta;
    let text = if reverse { meta.ticker_bottom } else { meta.ticker_top };
    
    let t1 = text.clone();
    let t2 = text.clone();

    view! {
        <div class="overflow-hidden py-3 border-b flex">
            <div class={format!("{} flex w-max items-center pr-4", track_class)}>
                <span class="font-sans text-[0.65rem] font-medium tracking-[0.2em] uppercase text-muted whitespace-nowrap pr-4">{t1.clone()}</span>
                <span class="font-sans text-[0.65rem] font-medium tracking-[0.2em] uppercase text-muted whitespace-nowrap pr-4">{t2.clone()}</span>
            </div>
            <div class={format!("{} flex w-max items-center pr-4", track_class)} aria-hidden="true">
                <span class="font-sans text-[0.65rem] font-medium tracking-[0.2em] uppercase text-muted whitespace-nowrap pr-4">{t1}</span>
                <span class="font-sans text-[0.65rem] font-medium tracking-[0.2em] uppercase text-muted whitespace-nowrap pr-4">{t2}</span>
            </div>
        </div>
    }
}
