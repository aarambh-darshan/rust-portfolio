use leptos::prelude::*;
use crate::content;

#[component]
pub fn Recognition() -> impl IntoView {
    let items = content::load().recognition;
    view! {
        <section class="recognition-section grid grid-cols-1 md:grid-cols-12 border-b">
            <div class="md:col-span-4 p-[clamp(32px,5vw,80px)] flex flex-col justify-center md:border-r">
                <h2 class="section-heading font-sans text-[0.7rem] font-semibold tracking-[0.25em] uppercase text-muted">"RECOGNITION"</h2>
                <p class="text-[0.85rem] text-muted leading-[1.7] mt-4 max-w-[360px]">
                    "Open-source impact, community growth, and technical authority
                    across GitHub, YouTube, and Medium."
                </p>
            </div>
            <div class="md:col-span-8 p-[clamp(32px,5vw,80px)] flex flex-col justify-center">
                <div class="flex flex-col">
                    {items.into_iter().map(|item| {
                        view! {
                            <div class="rec-item flex justify-between py-4.5 border-b last:border-b-0 hover-target">
                                <span class="text-[0.65rem] font-semibold tracking-[0.2em] text-muted">{item.org}</span>
                                <span class="font-serif text-[0.9rem] font-bold">{item.award}</span>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}