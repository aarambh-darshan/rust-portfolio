# MANIFESTO 2026

**DARSHAN VICHHI // SYSTEMS ENGINEER**

A brutalist, high-performance, single-page portfolio built from the ground up to reflect structural integrity, low latency, and systems architecture.

## Stack Architecture

- **Core:** Rust (Compiled to WebAssembly)
- **Framework:** Leptos v0.7 (Client-Side Rendered)
- **Styling:** Tailwind CSS v4 + Vanilla CSS
- **Animations:** GSAP (Timeline & ScrollTrigger) + Lenis Smooth Scroll
- **Toolchain:** Trunk

## Zero-Latency Static Deployment (GitHub Pages)

This project compiles into raw WebAssembly and static CSS/HTML. It requires absolutely no backend infrastructure, making it perfect for free hosting on GitHub Pages via Actions.

### How to Deploy:

1. Push this code to a repository (`aarambh-darshan/rust-portfolio`) on GitHub.
2. Go to your repository **Settings** > **Pages**.
3. Under **Build and deployment**, change the **Source** to **GitHub Actions**.
4. Create a new file in your repository at `.github/workflows/deploy.yml` with the following content to automatically build and deploy your WebAssembly portfolio:

```yaml
name: Deploy Trunk Build to GitHub Pages

on:
  push:
    branches: ["main"]

permissions:
  contents: read
  pages: write
  id-token: write

concurrency:
  group: "pages"
  cancel-in-progress: true

jobs:
  deploy:
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown

      - name: Install Trunk
        uses: jetli/trunk-action@v0.5.0
        with:
          version: 'latest'

      - name: Build
        run: trunk build --release --public-url /rust-portfolio/

      - name: Setup Pages
        uses: actions/configure-pages@v4

      - name: Upload artifact
        uses: actions/upload-pages-artifact@v3
        with:
          path: 'dist'

      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

Once this file is committed to your `main` branch, GitHub will automatically build your site and host it at `https://aarambh-darshan.github.io/rust-portfolio/`.

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
