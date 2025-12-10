// Copyright (c) 2025 Mykyta Voronyi. Licensed under MIT.

use leptos::prelude::*;
use crate::data::{
    SKILLS_CLOUD, SKILLS_DEVOPS, SKILLS_MONITORING,
    SKILLS_LANGUAGES, SKILLS_RUST, SKILLS_DB, SKILLS_SECURITY,
};

#[component]
fn SkillGroup(label: &'static str, skills: &'static [&'static str]) -> impl IntoView {
    view! {
        <div class="competency-group">
            <span class="competency-label">{label}</span>
            <div class="tags">
                {skills.iter().map(|s| view! {
                    <span class="tag">{*s}</span>
                }).collect_view()}
            </div>
        </div>
    }
}

#[component]
pub fn Competencies() -> impl IntoView {
    view! {
        <section class="section">
            <h2 class="section-title">"Core Competencies"</h2>
            <div class="competencies">
                <SkillGroup label="Cloud & Infrastructure" skills=SKILLS_CLOUD />
                <SkillGroup label="DevOps & Automation" skills=SKILLS_DEVOPS />
                <SkillGroup label="Monitoring" skills=SKILLS_MONITORING />
                <SkillGroup label="Languages" skills=SKILLS_LANGUAGES />
                <SkillGroup label="Rust Ecosystem" skills=SKILLS_RUST />
                <SkillGroup label="Databases" skills=SKILLS_DB />
                <SkillGroup label="Security" skills=SKILLS_SECURITY />
            </div>
        </section>
    }
}
