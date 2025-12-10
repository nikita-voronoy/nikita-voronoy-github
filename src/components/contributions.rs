// Copyright (c) 2025 Mykyta Voronyi. Licensed under MIT.

use leptos::prelude::*;
use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use crate::data::BUILD_TIMESTAMP;

const GITHUB_USERNAME: &str = "nikita-voronoy";
const CACHE_KEY: &str = "contributions_cache";

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CacheData {
    version: String,
    items: Vec<PullRequest>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct GithubSearchResponse {
    items: Vec<GithubItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct GithubItem {
    title: String,
    html_url: String,
    state: String,
    created_at: String,
    pull_request: Option<PullRequestInfo>,
    repository_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PullRequestInfo {
    merged_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct PullRequest {
    title: String,
    url: String,
    repo: String,
    repo_url: String,
    status: String,
    date: String,
}

fn extract_repo_name(repo_url: &str) -> String {
    repo_url
        .strip_prefix("https://api.github.com/repos/")
        .unwrap_or(repo_url)
        .to_string()
}

fn format_date(iso_date: &str) -> String {
    let date_part = iso_date.split('T').next().unwrap_or(iso_date);
    let parts: Vec<&str> = date_part.split('-').collect();
    if parts.len() == 3 {
        let month = match parts[1] {
            "01" => "Jan",
            "02" => "Feb",
            "03" => "Mar",
            "04" => "Apr",
            "05" => "May",
            "06" => "Jun",
            "07" => "Jul",
            "08" => "Aug",
            "09" => "Sep",
            "10" => "Oct",
            "11" => "Nov",
            "12" => "Dec",
            _ => parts[1],
        };
        let day = parts[2].trim_start_matches('0');
        format!("{} {}, {}", month, day, parts[0])
    } else {
        date_part.to_string()
    }
}

fn get_status(item: &GithubItem) -> String {
    if let Some(pr) = &item.pull_request {
        if pr.merged_at.is_some() {
            return "merged".to_string();
        }
    }
    item.state.clone()
}

fn load_from_cache() -> Option<Vec<PullRequest>> {
    let cached: Result<CacheData, _> = LocalStorage::get(CACHE_KEY);
    cached.ok().and_then(|data| {
        if data.version == BUILD_TIMESTAMP {
            Some(data.items)
        } else {
            None
        }
    })
}

fn save_to_cache(items: &[PullRequest]) {
    let data = CacheData {
        version: BUILD_TIMESTAMP.to_string(),
        items: items.to_vec(),
    };
    let _ = LocalStorage::set(CACHE_KEY, data);
}

async fn fetch_or_cache() -> Result<Vec<PullRequest>, String> {
    if let Some(cached) = load_from_cache() {
        return Ok(cached);
    }

    let url = format!(
        "https://api.github.com/search/issues?q=author:{}+type:pr+is:public+-user:{}&sort=created&order=desc&per_page=5",
        GITHUB_USERNAME, GITHUB_USERNAME
    );

    let response = Request::get(&url)
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("GitHub API error: {}", response.status()));
    }

    let data: GithubSearchResponse = response.json().await.map_err(|e| e.to_string())?;

    let items: Vec<PullRequest> = data
        .items
        .into_iter()
        .map(|item| {
            let status = get_status(&item);
            let repo = extract_repo_name(&item.repository_url);
            let repo_url = format!("https://github.com/{}", repo);
            PullRequest {
                title: item.title,
                url: item.html_url,
                repo,
                repo_url,
                status,
                date: format_date(&item.created_at),
            }
        })
        .collect();

    save_to_cache(&items);
    Ok(items)
}

#[component]
pub fn Contributions() -> impl IntoView {
    let contributions = LocalResource::new(|| fetch_or_cache());

    view! {
        <section class="section">
            <h2 class="section-title">"Open Source Contributions"</h2>
            <Suspense fallback=move || view! { <p class="loading">"Loading..."</p> }>
                {move || Suspend::new(async move {
                    match contributions.await {
                        Ok(items) if !items.is_empty() => {
                            view! {
                                <div class="contributions-list">
                                    {items.into_iter().map(|pr| {
                                        let status_class = format!("pr-status pr-{}", pr.status);
                                        view! {
                                            <div class="contribution-item">
                                                <a href={pr.url} target="_blank" class="pr-title">{pr.title}</a>
                                                <div class="pr-meta">
                                                    <a href={pr.repo_url} target="_blank" class="pr-repo">{pr.repo}</a>
                                                    <span class={status_class}>{pr.status.clone()}</span>
                                                    <span class="pr-date">{pr.date}</span>
                                                </div>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        }
                        Ok(_) => view! { <p>"No contributions found"</p> }.into_any(),
                        Err(e) => view! { <p class="error">{e}</p> }.into_any(),
                    }
                })}
            </Suspense>
        </section>
    }
}
