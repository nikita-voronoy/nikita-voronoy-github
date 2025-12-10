// Copyright (c) 2025 Mykyta Voronyi. Licensed under MIT.

use leptos::prelude::*;
use crate::data::{BUILD_VERSION, BUILD_COMMIT};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="footer">
            <span class="version">"v"{BUILD_VERSION}" ("{BUILD_COMMIT}")"</span>
        </footer>
    }
}
