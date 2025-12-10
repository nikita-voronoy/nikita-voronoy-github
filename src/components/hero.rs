// Copyright (c) 2025 Mykyta Voronyi. Licensed under MIT.

use leptos::prelude::*;
use crate::data::PROFILE;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section class="hero">
            <h1 class="name">{PROFILE.name}</h1>
            <p class="title">{PROFILE.title}</p>
            <p class="summary">{PROFILE.summary}</p>
        </section>
    }
}
