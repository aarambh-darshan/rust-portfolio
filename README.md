# MANIFESTO 2026

**DARSHAN VICHHI // SYSTEMS ENGINEER**

A brutalist, high-performance, single-page portfolio built from the ground up to reflect structural integrity, low latency, and systems architecture.

## Stack Architecture

- **Core:** Rust (Compiled to WebAssembly)
- **Framework:** Leptos v0.7 (Client-Side Rendered)
- **Styling:** Tailwind CSS v4 + Vanilla CSS
- **Animations:** GSAP (Timeline & ScrollTrigger) + Lenis Smooth Scroll
- **Toolchain:** Trunk

## Zero-Latency Static Deployment (Cloudflare Pages)

This project compiles into raw WebAssembly and static CSS/HTML. It requires absolutely no backend infrastructure, making it perfect for global Edge node distribution via Cloudflare Pages.

### How to Deploy:

1. Push this code to a repository (`aarambh-darshan/rust-portfolio`) on GitHub (Public or Private).
2. Log into the Cloudflare Dashboard -> **Workers & Pages**.
3. Click **Create Application** -> **Pages** -> **Connect to Git**.
4. Select your `rust-portfolio` repository.
5. In the "Set up builds and deployments" section, configure the following exactly:

**Build Settings:**
*   **Framework preset:** `None`
*   **Build command:** 
    ```bash
    curl -sLO https://github.com/trunk-rs/trunk/releases/download/v0.21.4/trunk-x86_64-unknown-linux-gnu.tar.gz && tar -xzf trunk-x86_64-unknown-linux-gnu.tar.gz && rustup target add wasm32-unknown-unknown && ./trunk build --release
    ```
*   **Build output directory:** `dist`

### Explanation of the Build Command

Because Cloudflare Pages runners are pristine Ubuntu containers, they have Rust installed (`rustc`, `cargo`), but they lack the `wasm32-unknown-unknown` target and the `trunk` CLI compiler. 
The command above dynamically fetches the latest pre-compiled `trunk` binary, extracts it, adds the WebAssembly target to the Rust toolchain, and finally executes `./trunk build --release` natively on the runner to generate the `dist` payload without bloating build minutes.

## Local Development

```bash
# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Install Trunk
cargo install trunk

# Run development server (with hot reload)
trunk serve --open
```

## Content Management (Zero-DB)

All content, statistics, and project links are strictly managed via `content.json`. 
During compile time (`trunk build`), Leptos parses this JSON file and embeds the statically typed structures directly into the binary overhead.

## License

Personal Project. All Rights Reserved.
