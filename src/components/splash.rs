use leptos::prelude::*;

#[component]
pub fn Splash() -> impl IntoView {
    view! {
        <div id="splash-screen"
             class="fixed inset-0 z-[9999] bg-bg flex flex-col items-center justify-center gap-10">
            <div class="stamp-container flex flex-col items-center gap-2">
                <div class="stamp-block stamp-1 font-serif font-black text-[clamp(2.5rem,7vw,5rem)] tracking-[-0.03em] leading-none text-text">"SYSTEMS"</div>
                <div class="stamp-block stamp-2 font-serif font-black text-[clamp(2.5rem,7vw,5rem)] tracking-[-0.03em] leading-none text-text">"ENGINEER"</div>
                <div class="stamp-block stamp-3 font-serif font-black text-[clamp(2.5rem,7vw,5rem)] tracking-[-0.03em] leading-none text-red">"DARSHAN"</div>
                <div class="stamp-block stamp-4 font-serif font-black text-[clamp(2.5rem,7vw,5rem)] tracking-[-0.03em] leading-none text-red">"VICHHI"</div>
            </div>
            <div class="splash-progress-ring w-[60px] h-[60px]">
                <svg viewBox="0 0 100 100" class="w-full h-full -rotate-90">
                    <circle class="ring-bg" cx="50" cy="50" r="45" />
                    <circle class="ring-fill" cx="50" cy="50" r="45" />
                </svg>
            </div>
        </div>
    }
}
