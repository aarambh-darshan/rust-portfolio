// src/components/experience.rs — Darshan Vichhi Portfolio × Leptos 0.7
//
// ── UI COMPONENT ─────────────────────────────────────────────────────────────
// Fully encapsulated component. Data is injected statically at compile time.
// ─────────────────────────────────────────────────────────────────────────────

use leptos::prelude::*;
use crate::content;

#[component]
pub fn Experience() -> impl IntoView {
    let entries = content::load().experience;
    view! {
        <section class="experience-section" id="about">
            <div class="grid grid-cols-12 border-b">
                <div class="col-span-12 p-[clamp(16px,2.5vw,32px)] flex flex-col justify-center border-b">
                    <h2 class="section-heading font-sans text-[0.7rem] font-semibold tracking-[0.25em] uppercase text-muted">"CURRICULUM VITAE"</h2>
                </div>
            </div>
            {entries.into_iter().map(|e| view! {
                <div class="cv-row grid grid-cols-1 md:grid-cols-12 border-b hover-target">
                    <div class="md:col-span-2 p-[clamp(16px,2.5vw,32px)] flex flex-col justify-center md:border-r font-sans text-[0.7rem] font-semibold tracking-[0.1em] text-red flex items-center">{e.years}</div>
                    <div class="md:col-span-4 p-[clamp(16px,2.5vw,32px)] flex flex-col justify-center md:border-r">
                        <h4 class="font-serif text-[clamp(1rem,1.5vw,1.3rem)] font-bold mb-1">{e.role}</h4>
                        <p class="text-[0.65rem] tracking-[0.15em] text-muted uppercase">{e.org}</p>
                    </div>
                    <div class="hidden md:flex md:col-span-6 p-[clamp(16px,2.5vw,32px)] flex-col justify-center">
                        <p class="text-[0.8rem] text-muted leading-[1.7]">{e.desc}</p>
                    </div>
                </div>
            }).collect_view()}
        </section>
    }
}