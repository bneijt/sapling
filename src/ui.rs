use leptos::prelude::*;

use crate::media::{encode_url_path, FolderEntry, VideoEntry};

const STYLE: &str = r#"
:root {
  color-scheme: light;
  --surface: #f3efe9;
  --panel: #fffdf8;
  --ink: #1f2a2f;
  --accent: #246a73;
  --muted: #6b7d83;
  --line: #d8d0c7;
}
* { box-sizing: border-box; }
body {
  margin: 0;
  font-family: "Atkinson Hyperlegible", "Trebuchet MS", sans-serif;
  color: var(--ink);
  background: radial-gradient(circle at top left, #fef8ef, #e4ecef 55%, #d7e3e7);
}
a { color: inherit; text-decoration: none; }
.shell {
  width: min(1100px, 96vw);
  margin: 1.5rem auto;
  background: color-mix(in srgb, var(--panel) 90%, white);
  border: 1px solid var(--line);
  border-radius: 16px;
  box-shadow: 0 12px 30px rgba(10, 30, 40, 0.08);
  overflow: hidden;
}
.header {
  padding: 1rem 1.2rem;
  border-bottom: 1px solid var(--line);
  display: flex;
  gap: 1rem;
  align-items: center;
  justify-content: space-between;
}
.header h1 { margin: 0; font-size: clamp(1.2rem, 1.5vw, 1.6rem); }
.path {
  color: var(--muted);
  font-size: 0.95rem;
  display: flex;
  flex-wrap: wrap;
  gap: 0.45rem;
}
.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(190px, 1fr));
  gap: 0.9rem;
  padding: 1rem;
}
.card {
  display: block;
  border: 1px solid var(--line);
  border-radius: 12px;
  background: #ffffffd8;
  overflow: hidden;
  transition: transform 0.18s ease, box-shadow 0.18s ease;
}
.card:hover {
  transform: translateY(-3px);
  box-shadow: 0 8px 18px rgba(25, 38, 49, 0.14);
}
.thumb {
  width: 100%;
  height: 132px;
  object-fit: cover;
  display: block;
  background: linear-gradient(135deg, #dae4e8, #f7efe5);
}
.thumb.placeholder {
  display: grid;
  place-items: center;
  color: var(--muted);
  font-weight: 700;
}
.meta { padding: 0.7rem; }
.meta h3 {
  margin: 0 0 0.3rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-size: 0.95rem;
}
.meta p {
  margin: 0;
  color: var(--muted);
  font-size: 0.8rem;
}
.video-wrap {
  padding: 1rem;
}
video {
  width: 100%;
  max-height: min(76vh, 800px);
  border-radius: 12px;
  border: 1px solid var(--line);
  background: #000;
}
.helper {
  padding: 1rem;
  color: var(--muted);
}
.cta {
    display: inline-block;
    padding: 0.55rem 0.9rem;
    border: 1px solid var(--line);
    border-radius: 999px;
    background: #fff;
    font-weight: 700;
}
@media (max-width: 640px) {
  .header { flex-direction: column; align-items: flex-start; }
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
                <main class="shell">
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
