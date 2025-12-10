// Copyright (c) 2025 Mykyta Voronyi. Licensed under MIT.

use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::data::{PROFILE, CONTACTS, Contact};
use crate::utils::download_pdf;

fn contact_icon(contact: &Contact) -> &'static str {
    match contact.platform {
        "Email" => "\u{1F4E9}",
        "Phone" => "\u{260E}",
        "LinkedIn" => "in",
        "Schedule" => "\u{1F4C5}",
        _ => "\u{1F517}",
    }
}

#[component]
pub fn FloatingCard() -> impl IntoView {
    let (can_download, set_can_download) = signal(false);
    let (honeypot, set_honeypot) = signal(String::new());
    let (is_open, set_is_open) = signal(false);

    Effect::new(move || {
        spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(2000).await;
            set_can_download.set(true);
        });
    });

    let on_download = move |_| {
        if !honeypot.get().is_empty() || !can_download.get() {
            return;
        }
        let filename = format!("{}_CV.pdf", PROFILE.name.replace(" ", "_"));
        download_pdf(&filename);
    };

    view! {
        <button
            class="floating-toggle"
            class:open=move || is_open.get()
            on:click=move |_| set_is_open.update(|v| *v = !*v)
        >
            {move || if is_open.get() { "\u{2715}" } else { "\u{2709}" }}
        </button>
        <div class="floating-card" class:open=move || is_open.get()>
            <input
                type="text"
                name="website"
                class="ohnohoney"
                tabindex="-1"
                autocomplete="off"
                on:input=move |ev| set_honeypot.set(event_target_value(&ev))
            />
            <button
                class="download-btn"
                class:disabled=move || !can_download.get()
                on:click=on_download
            >
                {move || if can_download.get() {
                    "Download Resume"
                } else {
                    "Loading..."
                }}
            </button>
            <div class="floating-divider"></div>
            <div class="contact-icons">
                {CONTACTS.iter().map(|c| view! {
                    <a href={c.url} target="_blank" class="contact-icon-link" title={c.label}>
                        {contact_icon(c)}
                    </a>
                }).collect_view()}
            </div>
        </div>
    }
}
