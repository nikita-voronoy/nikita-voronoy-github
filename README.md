# nikita-voronoy-github

Personal portfolio/resume website built with Rust, Leptos and WebAssembly.

## Development

### Prerequisites

- Rust with `wasm32-unknown-unknown` target
- [Trunk](https://trunkrs.dev/) - WASM bundler
- [Typst](https://typst.app/) - for PDF generation

### Install dependencies

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

### Run dev server

```bash
trunk serve
```

Open http://localhost:8080

## Resume

Edit `resume.yaml` to update resume content. Changes will be reflected on rebuild.

Structure:
- `profile` - name, title, summary
- `skills` - categorized skills (cloud, devops, languages, etc.)
- `experience` - work history
- `contacts` - contact links

PDF is auto-generated from `resume.yaml` via Typst during build.

## Deploy

Push to `main` branch triggers GitHub Actions workflow that builds and deploys to GitHub Pages.

## Links

- [Live Site](https://nikita-voronoy.github.io/nikita-voronoy-github/)
- [GitHub](https://github.com/nikita-voronoy)
