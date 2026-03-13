// src/main.rs — Darshan Vichhi Portfolio × Leptos 0.7
//
// ── HOW TO EDIT CONTENT ───────────────────────────────────────────────────────
// Edit /content.json — every section of the portfolio reads from that file.
// After saving, run `trunk serve` and the page will rebuild automatically.
// ─────────────────────────────────────────────────────────────────────────────

mod content;
mod gsap;
mod components;

use leptos::prelude::*;
use leptos_meta::*;
use wasm_bindgen::JsCast;

use components::{
    splash::Splash,
    header::Header,
    ticker::Ticker,
    hero::Hero,
    projects::Projects,
    philosophy::Philosophy,
    experience::Experience,
    skills::Skills,
    recognition::Recognition,
    insights::Insights,
    contact::Contact,
};

#[component]
fn App() -> impl IntoView {
    // Provide Meta Context for dynamic SEO updates
    provide_meta_context();


    // Boot animations after first render (~= DOMContentLoaded)
    Effect::new(|_| {
        let cb = wasm_bindgen::closure::Closure::once(move || {
            gsap::init_lenis();
            gsap::init_cursor();
            gsap::play_splash();
        });
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                200,
            )
            .unwrap();
        cb.forget();
    });

    view! {
        <Title text="DARSHAN VICHHI [MANIFESTO] — Systems Engineer" />
        <Meta name="description" content="Darshan Vichhi — Polyglot Systems Programmer specialize in high-throughput, memory-safe architectures." />

        <Splash />

        <main id="smooth-wrapper">
            <Header />

            <Ticker />

            <Hero />

            <Projects />

            <Ticker reverse=true />

            <Philosophy />

            <Experience />

            <Skills />

            <Recognition />

            <Insights />

            <Contact />
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}