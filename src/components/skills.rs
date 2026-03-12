use crate::content;
use leptos::prelude::*;

#[component]
pub fn Skills() -> impl IntoView {
    let s = content::load().skills;
    let marquee = s.marquee.clone();
    let m2 = marquee.clone();
    view! {
        <section class="skills-section grid grid-cols-2 md:grid-cols-12 border-b">
            <div class="col-span-full p-[clamp(16px,2.5vw,32px)] flex flex-col justify-center border-b">
                <h2 class="section-heading font-sans text-[0.7rem] font-semibold tracking-[0.25em] uppercase text-muted">"APPENDIX A: COMPETENCIES"</h2>
            </div>
            {s.buckets.into_iter().map(|b| {
                let cls = match b.color.as_str() {
                    "blue"   => "col-span-1 md:col-span-3 p-[clamp(16px,2.5vw,32px)] flex flex-col justify-center border-r border-b bg-blue text-white",
                    "yellow" => "col-span-1 md:col-span-3 p-[clamp(16px,2.5vw,32px)] flex flex-col justify-center border-r border-b bg-yellow text-black",
                    _        => "col-span-1 md:col-span-3 p-[clamp(16px,2.5vw,32px)] flex flex-col justify-center border-r border-b",
                };
                view! {
                    <div class=cls>
                        <h3 class="font-sans text-[0.65rem] font-bold tracking-[0.2em] mb-3">{b.title}</h3>
                        <p class="text-[0.85rem] leading-[1.7]">{b.list}</p>
                    </div>
                }
            }).collect_view()}
            <div class="col-span-full p-[clamp(16px,2.5vw,32px)] flex flex-col justify-center">
                <div class="overflow-hidden py-5 flex">
                    <div class="ticker-track flex w-max items-center pr-4">
                        <span class="font-serif text-[clamp(2rem,4vw,3.5rem)] font-black whitespace-nowrap pl-4">{marquee.clone()}</span>
                        <span class="marquee-outline font-serif text-[clamp(2rem,4vw,3.5rem)] font-black whitespace-nowrap pl-4">{m2.clone()}</span>
                    </div>
                    <div class="ticker-track flex w-max items-center pr-4" aria-hidden="true">
                        <span class="font-serif text-[clamp(2rem,4vw,3.5rem)] font-black whitespace-nowrap pl-4">{marquee.clone()}</span>
                        <span class="marquee-outline font-serif text-[clamp(2rem,4vw,3.5rem)] font-black whitespace-nowrap pl-4">{m2}</span>
                    </div>
                </div>
            </div>
        </section>
    }
}
