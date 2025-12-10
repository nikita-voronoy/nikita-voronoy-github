// Copyright (c) 2025 Mykyta Voronyi. Licensed under MIT.

use leptos::prelude::*;
use crate::data::EXPERIENCE;

#[component]
fn TerminalCard(
    company: &'static str,
    position: &'static str,
    period: &'static str,
    location: &'static str,
    highlights: &'static [&'static str],
) -> impl IntoView {
    view! {
        <div class="terminal-card">
            <div class="terminal-header">
                <span class="terminal-btn red"></span>
                <span class="terminal-btn yellow"></span>
                <span class="terminal-btn green"></span>
                <span class="terminal-title">{company}" — "{position}</span>
            </div>
            <div class="terminal-body">
                <div class="terminal-line">
                    <span class="prompt">"$"</span>
                    <span class="cmd">" cat info.txt"</span>
                </div>
                <div class="terminal-output">
                    <span class="output-label">"Period: "</span>{period}<br/>
                    <span class="output-label">"Location: "</span>{location}
                </div>
                <div class="terminal-line">
                    <span class="prompt">"$"</span>
                    <span class="cmd">" cat achievements.md"</span>
                </div>
                <ul class="terminal-list">
                    {highlights.iter().map(|h| view! {
                        <li>{*h}</li>
                    }).collect_view()}
                </ul>
            </div>
        </div>
    }
}

#[component]
pub fn Experience() -> impl IntoView {
    view! {
        <section class="section">
            <h2 class="section-title">"Professional Experience"</h2>
            <div class="timeline">
                {EXPERIENCE.iter().map(|exp| view! {
                    <TerminalCard
                        company=exp.company
                        position=exp.position
                        period=exp.period
                        location=exp.location
                        highlights=exp.highlights
                    />
                }).collect_view()}
            </div>
        </section>
    }
}
