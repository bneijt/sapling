use leptos::prelude::*;

use crate::media::{encode_url_path, FolderEntry, VideoEntry};

const STYLE: &str = r#"
:root {
  color-scheme: light;
  --surface: #f3efe9;
    --bg-0: #4d4d4d;
    --bg-1: #3c3c3c;
    --bg-2: #2f2f2f;
    --panel: #5a5a5a;
    --panel-hover: #666666;
    --ink: #f2f2f2;
    --muted: #c3c3c3;
    --line: #707070;
* { box-sizing: border-box; }
body {
html {
    min-height: 100%;
    background: linear-gradient(180deg, var(--bg-0), var(--bg-1) 18%, var(--bg-2));
}
  margin: 0;
  font-family: "Atkinson Hyperlegible", "Trebuchet MS", sans-serif;
  color: var(--ink);
  background: radial-gradient(circle at top left, #fef8ef, #e4ecef 55%, #d7e3e7);
    background:
        linear-gradient(180deg, rgba(255, 255, 255, 0.03), transparent 18%),
        repeating-linear-gradient(
            0deg,
            rgba(255, 255, 255, 0.015) 0,
            rgba(255, 255, 255, 0.015) 1px,
            transparent 1px,
            transparent 32px
        ),
        linear-gradient(180deg, var(--bg-0), var(--bg-1) 18%, var(--bg-2));
a { color: inherit; text-decoration: none; }
.shell {
.app {
    min-height: 100vh;
.header {
  padding: 1rem 1.2rem;
    padding: 0.85rem 1rem;
    border-bottom: 1px solid var(--line);
    background: linear-gradient(180deg, #606060, #505050);
  gap: 1rem;
  align-items: center;
  justify-content: space-between;
}
.header h1 { margin: 0; font-size: clamp(1.2rem, 1.5vw, 1.6rem); }
.header h1 {
    margin: 0;
    font-size: clamp(1rem, 1.3vw, 1.35rem);
    font-weight: 700;
    letter-spacing: 0.02em;
}
  color: var(--muted);
  font-size: 0.95rem;
    font-size: 0.88rem;
  flex-wrap: wrap;
  gap: 0.45rem;
}
.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(190px, 1fr));
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
    background: linear-gradient(180deg, #5d5d5d, #535353);
  overflow: hidden;
    min-width: 0;
    transition: background 0.18s ease;
}
.card:hover {
    background: linear-gradient(180deg, #6a6a6a, #5b5b5b);
}
.thumb {
  width: 100%;
    aspect-ratio: 16 / 9;
  object-fit: cover;
  display: block;
    background: linear-gradient(135deg, #6c6c6c, #4d4d4d);
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
    border-top: 1px solid rgba(255, 255, 255, 0.05);
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
    background: rgba(0, 0, 0, 0.08);
}
.cta {
    display: inline-block;
    padding: 0.5rem 0.85rem;
    border: 1px solid var(--line);
    background: linear-gradient(180deg, #676767, #585858);
    font-weight: 700;
    color: var(--ink);
}
@media (max-width: 640px) {
  .header { flex-direction: column; align-items: flex-start; }
    .grid { grid-template-columns: repeat(auto-fill, minmax(160px, 1fr)); }
}
"#;

fn page_shell(title: &'static str, path: String, content: impl IntoView + 'static) -> String {
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
                        <h1>"Sapling Media"</h1>
                        <div class="path">{path}</div>
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
            view! {
                <a class="card" href=play_href>
                    <div class="thumb placeholder">"Video"</div>
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
        view! {
            <section class="helper">{breadcrumb_view}</section>
            <section class="grid">{folder_cards}{video_cards}</section>
        },
    )
}

pub fn render_video_page(display_name: String, media_src: String, parent_href: String) -> String {
    page_shell(
        "Now Playing",
        display_name.clone(),
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
        view! {
            <section class="helper">
                <h2>"Not found"</h2>
                <p>{message}</p>
            </section>
        },
    )
}
