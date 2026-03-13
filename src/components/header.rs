// src/components/header.rs — Darshan Vichhi Portfolio × Leptos 0.7
//
// ── UI COMPONENT ─────────────────────────────────────────────────────────────
// Fully encapsulated component. Data is injected statically at compile time.
// ─────────────────────────────────────────────────────────────────────────────

use leptos::prelude::*;
use crate::{content, gsap::lenis_scroll_to};

#[component]
pub fn Header() -> impl IntoView {
    let meta = content::load().meta;
    let scroll = |sel: &'static str| move |_: web_sys::MouseEvent| lenis_scroll_to(sel);
    view! {
        <header class="ed-header sticky top-0 z-100 bg-bg grid grid-cols-2 md:grid-cols-12">
            <div class="hidden md:flex md:col-span-3 p-[clamp(16px,2.5vw,32px)] flex-col justify-center border-r border-b">
                <span class="font-sans text-[0.6rem] font-medium tracking-[0.2em] uppercase text-muted">{meta.issue}</span>
            </div>
            <div class="col-span-1 md:col-span-6 p-[clamp(16px,2.5vw,32px)] flex justify-center items-center border-r border-b">
                <span class="logo font-serif text-[clamp(0.8rem,1.2vw,1rem)] font-bold tracking-[0.15em] hover-target">"DARSHAN VICHHI"</span>
            </div>
            <nav class="col-span-1 md:col-span-3 p-[clamp(16px,2.5vw,32px)] flex flex-row gap-5 items-center justify-center border-b" aria-label="Main Navigation">
                <a class="nav-link text-[0.65rem] font-semibold tracking-[0.15em] uppercase hover-target" href="#work"    on:click=scroll("#work")   >"Archive"</a>
                <a class="nav-link text-[0.65rem] font-semibold tracking-[0.15em] uppercase hover-target" href="#about"   on:click=scroll("#about")  >"Index"</a>
                <a class="nav-link text-[0.65rem] font-semibold tracking-[0.15em] uppercase hover-target" href="#contact" on:click=scroll("#contact")>"Uplink"</a>
            </nav>
        </header>
    }
}