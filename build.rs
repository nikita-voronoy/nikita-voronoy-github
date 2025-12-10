// Copyright (c) 2025 Mykyta Voronyi. Licensed under MIT.

use serde::Deserialize;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

#[derive(Deserialize)]
struct Resume {
    profile: Profile,
    skills: Skills,
    experience: Vec<Experience>,
    contacts: Vec<Contact>,
}

#[derive(Deserialize)]
struct Profile {
    name: String,
    title: String,
    summary: String,
}

#[derive(Deserialize)]
struct Skills {
    cloud: Vec<String>,
    devops: Vec<String>,
    monitoring: Vec<String>,
    languages: Vec<String>,
    rust: Vec<String>,
    databases: Vec<String>,
    security: Vec<String>,
}

#[derive(Deserialize)]
struct Experience {
    company: String,
    position: String,
    period: String,
    location: String,
    highlights: Vec<String>,
}

#[derive(Deserialize)]
struct Contact {
    platform: String,
    url: String,
    label: String,
}

fn main() {
    println!("cargo:rerun-if-changed=resume.yaml");
    println!("cargo:rerun-if-changed=.git/HEAD");

    let yaml_content = fs::read_to_string("resume.yaml").expect("Failed to read resume.yaml");
    let resume: Resume = serde_yaml::from_str(&yaml_content).expect("Failed to parse resume.yaml");

    generate_rust_code(&resume);
    generate_build_info();
    generate_pdf(&resume);
}

fn generate_build_info() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest_path = Path::new(&out_dir).join("build_info.rs");

    let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".to_string());

    let commit = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "dev".to_string());

    let commit = if commit.is_empty() { "dev".to_string() } else { commit };

    let content = format!(
        "pub const BUILD_VERSION: &str = {:?};\npub const BUILD_COMMIT: &str = {:?};\npub const BUILD_TIMESTAMP: &str = {:?};\n",
        version, commit, commit
    );

    let existing = fs::read_to_string(&dest_path).unwrap_or_default();
    if existing != content {
        fs::write(&dest_path, content).expect("Failed to write build_info.rs");
    }
}

fn generate_rust_code(resume: &Resume) {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest_path = Path::new(&out_dir).join("resume_data.rs");
    let mut file = fs::File::create(&dest_path).expect("Failed to create resume_data.rs");

    writeln!(file, "pub struct Profile {{").ok();
    writeln!(file, "    pub name: &'static str,").ok();
    writeln!(file, "    pub title: &'static str,").ok();
    writeln!(file, "    pub summary: &'static str,").ok();
    writeln!(file, "}}").ok();
    writeln!(file).ok();

    writeln!(file, "pub struct Experience {{").ok();
    writeln!(file, "    pub company: &'static str,").ok();
    writeln!(file, "    pub position: &'static str,").ok();
    writeln!(file, "    pub period: &'static str,").ok();
    writeln!(file, "    pub location: &'static str,").ok();
    writeln!(file, "    pub highlights: &'static [&'static str],").ok();
    writeln!(file, "}}").ok();
    writeln!(file).ok();

    writeln!(file, "pub struct Contact {{").ok();
    writeln!(file, "    pub platform: &'static str,").ok();
    writeln!(file, "    pub url: &'static str,").ok();
    writeln!(file, "    pub label: &'static str,").ok();
    writeln!(file, "}}").ok();
    writeln!(file).ok();

    writeln!(file, "pub const PROFILE: Profile = Profile {{").ok();
    writeln!(file, "    name: {:?},", resume.profile.name).ok();
    writeln!(file, "    title: {:?},", resume.profile.title).ok();
    writeln!(file, "    summary: {:?},", resume.profile.summary).ok();
    writeln!(file, "}};").ok();
    writeln!(file).ok();

    write_skills_const(&mut file, "SKILLS_CLOUD", &resume.skills.cloud);
    write_skills_const(&mut file, "SKILLS_DEVOPS", &resume.skills.devops);
    write_skills_const(&mut file, "SKILLS_MONITORING", &resume.skills.monitoring);
    write_skills_const(&mut file, "SKILLS_LANGUAGES", &resume.skills.languages);
    write_skills_const(&mut file, "SKILLS_RUST", &resume.skills.rust);
    write_skills_const(&mut file, "SKILLS_DB", &resume.skills.databases);
    write_skills_const(&mut file, "SKILLS_SECURITY", &resume.skills.security);

    writeln!(file, "pub const EXPERIENCE: &[Experience] = &[").ok();
    for exp in &resume.experience {
        writeln!(file, "    Experience {{").ok();
        writeln!(file, "        company: {:?},", exp.company).ok();
        writeln!(file, "        position: {:?},", exp.position).ok();
        writeln!(file, "        period: {:?},", exp.period).ok();
        writeln!(file, "        location: {:?},", exp.location).ok();
        writeln!(file, "        highlights: &[").ok();
        for h in &exp.highlights {
            writeln!(file, "            {:?},", h).ok();
        }
        writeln!(file, "        ],").ok();
        writeln!(file, "    }},").ok();
    }
    writeln!(file, "];").ok();
    writeln!(file).ok();

    writeln!(file, "pub const CONTACTS: &[Contact] = &[").ok();
    for contact in &resume.contacts {
        writeln!(file, "    Contact {{").ok();
        writeln!(file, "        platform: {:?},", contact.platform).ok();
        writeln!(file, "        url: {:?},", contact.url).ok();
        writeln!(file, "        label: {:?},", contact.label).ok();
        writeln!(file, "    }},").ok();
    }
    writeln!(file, "];").ok();
}

fn write_skills_const(file: &mut fs::File, name: &str, skills: &[String]) {
    write!(file, "pub const {}: &[&str] = &[", name).ok();
    for (i, skill) in skills.iter().enumerate() {
        if i > 0 {
            write!(file, ", ").ok();
        }
        write!(file, "{:?}", skill).ok();
    }
    writeln!(file, "];").ok();
}

fn generate_pdf(resume: &Resume) {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let typ_path = Path::new(&out_dir).join("resume.typ");
    let pdf_path = Path::new("assets/resume.pdf");

    let mut typ_content = String::new();

    typ_content.push_str("#set page(margin: 1.5cm)\n");
    typ_content.push_str("#set text(font: \"Liberation Sans\", size: 9pt)\n");
    typ_content.push_str("#show link: it => underline(text(fill: rgb(\"#0066cc\"), it))\n\n");

    typ_content.push_str(&format!(
        "#align(center)[#text(size: 18pt, weight: \"bold\")[{}]]\n",
        escape_typst(&resume.profile.name)
    ));
    typ_content.push_str(&format!(
        "#align(center)[#text(size: 11pt)[{}]]\n\n",
        escape_typst(&resume.profile.title)
    ));
    typ_content.push_str(&format!("{}\n\n", escape_typst(&resume.profile.summary)));

    typ_content.push_str("#text(size: 12pt, weight: \"bold\")[CORE COMPETENCIES]\n\n");

    let skills_sections = [
        ("Cloud & Infrastructure", &resume.skills.cloud),
        ("DevOps & Automation", &resume.skills.devops),
        ("Monitoring & Observability", &resume.skills.monitoring),
        ("Programming Languages", &resume.skills.languages),
        ("Rust Ecosystem", &resume.skills.rust),
        ("Databases & Messaging", &resume.skills.databases),
        ("Security", &resume.skills.security),
    ];

    for (label, skills) in skills_sections {
        typ_content.push_str(&format!(
            "*{}:* {}\n\n",
            label,
            skills.iter().map(|s| escape_typst(s)).collect::<Vec<_>>().join(", ")
        ));
    }

    typ_content.push_str("#text(size: 12pt, weight: \"bold\")[PROFESSIONAL EXPERIENCE]\n\n");

    for exp in &resume.experience {
        typ_content.push_str(&format!(
            "#text(size: 10pt, weight: \"bold\")[{} — {}]\n",
            escape_typst(&exp.company),
            escape_typst(&exp.position)
        ));
        typ_content.push_str(&format!(
            "#text(style: \"italic\")[{} | {}]\n\n",
            escape_typst(&exp.period),
            escape_typst(&exp.location)
        ));
        for highlight in &exp.highlights {
            typ_content.push_str(&format!("- {}\n", escape_typst(highlight)));
        }
        typ_content.push_str("\n");
    }

    typ_content.push_str("#text(size: 12pt, weight: \"bold\")[CONTACT]\n\n");

    for contact in &resume.contacts {
        typ_content.push_str(&format!(
            "{}: #link(\"{}\")[{}]\n\n",
            escape_typst(&contact.platform),
            &contact.url,
            escape_typst(&contact.label)
        ));
    }

    fs::write(&typ_path, &typ_content).expect("Failed to write resume.typ");

    let status = Command::new("typst")
        .args(["compile", typ_path.to_str().unwrap(), pdf_path.to_str().unwrap()])
        .status()
        .expect("Failed to run typst. Is typst-cli installed?");

    if !status.success() {
        panic!("typst compile failed");
    }

    println!("cargo:warning=Generated resume.pdf with typst");
}

fn escape_typst(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('#', "\\#")
        .replace('$', "\\$")
        .replace('@', "\\@")
        .replace('[', "\\[")
        .replace(']', "\\]")
        .replace('"', "\\\"")
}
