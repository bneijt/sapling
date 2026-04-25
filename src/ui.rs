use leptos::prelude::*;

use crate::media::{encode_url_path, FolderEntry, SearchEntry, SearchEntryKind, VideoEntry};

const STYLE: &str = r#"
:root {
    color-scheme: light;
    --bg: #f3f4f6;
    --panel: #ffffff;
    --panel-hover: #f8fafc;
    --header: #ffffff;
    --ink: #111827;
    --muted: #6b7280;
    --line: #d1d5db;
}

* { box-sizing: border-box; }

html,
body {
    min-height: 100%;
}

body {
    margin: 0;
    font-family: "Atkinson Hyperlegible", "Trebuchet MS", sans-serif;
    color: var(--ink);
    background: var(--bg);
}

a { color: inherit; text-decoration: none; }

.app {
    min-height: 100vh;
}

.header {
    padding: 0.85rem 1rem;
    border-bottom: 1px solid var(--line);
    background: var(--header);
    display: flex;
    gap: 0.8rem;
    align-items: center;
    justify-content: flex-end;
}

.path {
    color: var(--muted);
    font-size: 0.88rem;
    display: flex;
    flex-wrap: wrap;
    gap: 0.45rem;
}

.header-right {
    display: flex;
    align-items: center;
    gap: 0.8rem;
    min-width: min(72vw, 760px);
    width: 100%;
}

.search-tools {
    margin-left: auto;
    display: flex;
    align-items: center;
    gap: 0.4rem;
}

.home-button {
    width: auto;
    height: 2.1rem;
    border: 1px solid var(--line);
    background: #ffffff;
    color: var(--ink);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0 0.55rem;
    font-size: 0.95rem;
    line-height: 1;
}

.home-button:hover {
    background: var(--panel-hover);
}

.search-form {
    margin: 0;
    width: min(420px, 100%);
}

.search-input {
    width: 100%;
    height: 2.1rem;
    border: 1px solid var(--line);
    background: #ffffff;
    color: var(--ink);
    padding: 0 0.7rem;
    font: inherit;
}

.search-input:focus {
    outline: 2px solid #c7d2fe;
    outline-offset: 0;
}

.grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
    gap: 1px;
    background: var(--line);
    border-top: 1px solid var(--line);
    border-bottom: 1px solid var(--line);
}

.grid:empty {
    border-bottom: none;
}

.card {
    display: block;
    background: var(--panel);
    overflow: hidden;
    min-width: 0;
    transition: background 0.18s ease;
}

.card:hover {
    background: var(--panel-hover);
}

.thumb {
    width: 100%;
    aspect-ratio: 16 / 9;
    object-fit: cover;
    display: block;
    background: #e5e7eb;
}

.thumb.placeholder {
    display: grid;
    place-items: center;
    color: var(--muted);
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
}

.meta {
    padding: 0.65rem 0.8rem 0.8rem;
    border-top: 1px solid var(--line);
}

.meta h3 {
    margin: 0 0 0.3rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    font-size: 0.9rem;
    font-weight: 700;
}

.meta p {
    margin: 0;
    color: var(--muted);
    font-size: 0.76rem;
}

.video-wrap {
    padding: 0.8rem 1rem;
}

video {
    width: 100%;
    max-height: calc(100vh - 8rem);
    background: #000;
}

.helper {
    padding: 0.8rem 1rem;
    color: var(--muted);
    background: #f9fafb;
}

.cta {
    display: inline-block;
    padding: 0.5rem 0.85rem;
    border: 1px solid var(--line);
    background: #ffffff;
    font-weight: 700;
    color: var(--ink);
}

.results {
    border-top: 1px solid var(--line);
    border-bottom: 1px solid var(--line);
    background: var(--panel);
}

.results-header {
    padding: 0.75rem 1rem;
    border-bottom: 1px solid var(--line);
    color: var(--muted);
    font-size: 0.86rem;
}

.results-list {
    list-style: none;
    margin: 0;
    padding: 0;
}

.results-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.6rem;
    padding: 0.72rem 1rem;
    border-top: 1px solid #eceff3;
}

.results-row:first-child {
    border-top: none;
}

.results-kind {
    color: var(--muted);
    font-size: 0.74rem;
    text-transform: uppercase;
    letter-spacing: 0.07em;
}

.results-path {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.results-empty {
    padding: 1rem;
    color: var(--muted);
}

@media (max-width: 640px) {
    .header { flex-direction: column; align-items: stretch; }
    .header-right { width: 100%; min-width: 0; flex-direction: column; align-items: stretch; }
    .search-tools { margin-left: 0; width: 100%; }
    .search-form { width: 100%; }
    .grid { grid-template-columns: repeat(auto-fill, minmax(160px, 1fr)); }
}
"#;

fn page_shell(
    title: &'static str,
    path: String,
    search_query: String,
    content: impl IntoView + 'static,
) -> String {
    let html = view! {
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <title>{title}</title>
                <style>{STYLE}</style>
            </head>
            <body>
                <main class="app">
                    <header class="header">
                        <div class="header-right">
                            <div class="path">{path}</div>
                            <div class="search-tools">
                                <a class="home-button" href="/browse/" aria-label="Home">"⌂"</a>
                                <form class="search-form" action="/browse/" method="get">
                                    <input
                                        class="search-input"
                                        type="search"
                                        name="q"
                                        placeholder="Search all paths"
                                        value=search_query
                                    />
                                </form>
                            </div>
                        </div>
                    </header>
                    {content}
                </main>
            </body>
        </html>
    };

    format!("<!DOCTYPE html>{}", html.to_html())
}

pub fn render_browse_page(
    breadcrumbs: &[(String, String)],
    folders: &[FolderEntry],
    videos: &[VideoEntry],
) -> String {
    let breadcrumb_view = breadcrumbs
        .iter()
        .enumerate()
        .flat_map(|(idx, (label, href))| {
            let is_last = idx + 1 == breadcrumbs.len();
            if is_last {
                vec![view! { <span>{label.clone()}</span> }.into_any()]
            } else {
                vec![
                    view! { <a href=href.clone()>{label.clone()}</a> }.into_any(),
                    view! { <span>"/"</span> }.into_any(),
                ]
            }
        })
        .collect_view();

    let folder_cards = folders
        .iter()
        .map(|folder| {
            let browse_href = format!("/browse/{}", encode_url_path(&folder.relative_path));
            let thumb_view = match &folder.thumbnail_relative_path {
                Some(thumbnail) => {
                    let src = format!("/media/{}", encode_url_path(thumbnail));
                    view! { <img class="thumb" src=src alt=folder.name.clone()/> }.into_any()
                }
                None => view! { <div class="thumb placeholder">"Folder"</div> }.into_any(),
            };

            view! {
                <a class="card" href=browse_href>
                    {thumb_view}
                    <div class="meta">
                        <h3>{folder.name.clone()}</h3>
                        <p>"Folder"</p>
                    </div>
                </a>
            }
        })
        .collect_view();

    let video_cards = videos
        .iter()
        .map(|video| {
            let play_href = format!("/play/{}", encode_url_path(&video.relative_path));
            let thumb_view = match &video.thumbnail_url {
                Some(thumbnail_url) => {
                    view! { <img class="thumb" src=thumbnail_url.clone() alt=video.name.clone()/> }.into_any()
                }
                None => view! { <div class="thumb placeholder">"Video"</div> }.into_any(),
            };

            view! {
                <a class="card" href=play_href>
                    {thumb_view}
                    <div class="meta">
                        <h3>{video.name.clone()}</h3>
                        <p>"Play in browser"</p>
                    </div>
                </a>
            }
        })
        .collect_view();

    page_shell(
        "Sapling Media",
        String::new(),
        String::new(),
        view! {
            <section class="helper">{breadcrumb_view}</section>
            <section class="grid">{folder_cards}{video_cards}</section>
        },
    )
}

pub fn render_search_results_page(query: &str, entries: &[SearchEntry]) -> String {
    let listing = if entries.is_empty() {
        view! { <div class="results-empty">"No matches found."</div> }.into_any()
    } else {
        entries
            .iter()
            .map(|entry| {
                let href = match entry.kind {
                    SearchEntryKind::Folder => {
                        format!("/browse/{}", encode_url_path(&entry.relative_path))
                    }
                    SearchEntryKind::Video => format!("/play/{}", encode_url_path(&entry.relative_path)),
                };
                let kind = match entry.kind {
                    SearchEntryKind::Folder => "Folder",
                    SearchEntryKind::Video => "Video",
                };

                view! {
                    <li class="results-row">
                        <a class="results-path" href=href>{entry.relative_path.to_string_lossy().to_string()}</a>
                        <span class="results-kind">{kind}</span>
                    </li>
                }
            })
            .collect_view()
            .into_any()
    };

    page_shell(
        "Search Results",
        format!("Search: {}", query),
        query.to_string(),
        view! {
            <section class="results">
                <div class="results-header">{format!("{} match(es)", entries.len())}</div>
                {if entries.is_empty() {
                    listing
                } else {
                    view! { <ol class="results-list">{listing}</ol> }.into_any()
                }}
            </section>
        },
    )
}

pub fn render_video_page(display_name: String, media_src: String, parent_href: String) -> String {
    page_shell(
        "Now Playing",
        display_name.clone(),
        String::new(),
        view! {
            <section class="video-wrap">
                <a class="cta" href=parent_href>
                    "Back to folder"
                </a>
            </section>
            <section class="video-wrap">
                <video controls=true autoplay=true preload="metadata">
                    <source src=media_src/>
                    "Your browser cannot play this video format natively."
                </video>
            </section>
        },
    )
}

pub fn render_not_found(message: String) -> String {
    page_shell(
        "Not Found",
        "Error".to_string(),
        String::new(),
        view! {
            <section class="helper">
                <h2>"Not found"</h2>
                <p>{message}</p>
            </section>
        },
    )
}
