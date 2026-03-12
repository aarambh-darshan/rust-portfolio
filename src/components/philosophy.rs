use leptos::prelude::*;

#[component]
pub fn Philosophy() -> impl IntoView {
    view! {
        <section class="philosophy-section grid grid-cols-1 md:grid-cols-12 border-b">
            // Left: big red block with Bauhaus title
            <div class="md:col-span-8 p-[clamp(32px,5vw,80px)] flex flex-col justify-center border-r bg-red text-white">
                <h2 class="hero-title font-serif text-[clamp(3rem,8vw,7.5rem)] font-black leading-[0.95] tracking-[-0.03em] text-center text-white">
                    "Form"<br />"Follows"<br /><span class="italic">"Function."</span>
                </h2>
            </div>

            // Right: geometric shapes + body
            <div class="md:col-span-4 p-[clamp(32px,5vw,80px)] flex flex-col justify-center bg-offwhite text-text">
                <div class="geometric-deco flex gap-5 mb-7">
                    <div class="geo-circle w-[60px] h-[60px] rounded-full bg-red"></div>
                    <div class="geo-square w-[60px] h-[60px] bg-blue"></div>
                </div>
                <p class="text-[0.9rem] leading-[1.8] text-text max-w-[360px]">
                    "Every line of code serves a purpose. No excess. No bloat.
                    Only pure, unadulterated throughput and resilience."
                </p>
            </div>
        </section>
    }
}
