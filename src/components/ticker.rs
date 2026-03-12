use leptos::prelude::*;

#[component]
pub fn Ticker(
    #[prop(into)] text: String,
    #[prop(default = false)] reverse: bool,
) -> impl IntoView {
    let track_class = if reverse { "ticker-track reverse" } else { "ticker-track" };
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
