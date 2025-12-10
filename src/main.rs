// Copyright (c) 2025 Mykyta Voronyi. Licensed under MIT.

mod data;
mod components;
mod utils;

use leptos::prelude::*;
use components::{Hero, Competencies, Experience, Contributions, FloatingCard, Footer};

fn main() {
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <main class="container">
            <Hero />
            <Competencies />
            <Experience />
            <Contributions />
            <FloatingCard />
        </main>
        <Footer />
    }
}
