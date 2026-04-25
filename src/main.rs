mod media;
mod thumbnail;
mod ui;

use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::get;
use axum::Router;
use clap::Parser;
use tower_http::services::ServeDir;

use crate::media::{
    encode_url_path, format_breadcrumbs, resolve_directory, resolve_video_file, scan_directory, ResolveError,
};
use crate::thumbnail::valid_thumbnail_path_for_video;

#[derive(Debug, Parser)]
#[command(name = "sapling")]
#[command(about = "Small read-only media server with a Leptos UI")]
struct Cli {
    #[arg(long, short = 'm', value_name = "PATH")]
    media_root: PathBuf,
    #[arg(long, default_value = "127.0.0.1")]
    bind: IpAddr,
    #[arg(long, default_value_t = 3000)]
    port: u16,
}

#[derive(Clone)]
struct AppState {
    media_root: Arc<PathBuf>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let media_root = std::fs::canonicalize(&cli.media_root)?;
    if !media_root.is_dir() {
        anyhow::bail!("--media-root must point to an existing directory");
    }

    let state = AppState {
        media_root: Arc::new(media_root.clone()),
    };

    let media_service = ServeDir::new(media_root);
    let app = Router::new()
        .route("/", get(root))
        .route("/browse/", get(browse_root))
        .route("/browse/{*path}", get(browse))
        .route("/play/{*path}", get(play))
        .route("/thumb/video/{*path}", get(video_thumbnail))
        .nest_service("/media", media_service)
        .with_state(state);

    let addr = SocketAddr::new(cli.bind, cli.port);
    println!("Sapling listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn root() -> Redirect {
    Redirect::to("/browse/")
}

async fn browse_root(State(state): State<AppState>) -> Response {
    browse(State(state), Path(String::new())).await
}

async fn browse(State(state): State<AppState>, Path(path): Path<String>) -> Response {
    let (absolute, relative) = match resolve_directory(state.media_root.as_ref(), &path) {
        Ok(values) => values,
        Err(err) => return render_path_error(err),
    };

    let (folders, videos) = match scan_directory(state.media_root.as_ref(), &absolute, &relative) {
        Ok(values) => values,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html(ui::render_not_found(format!("Could not read directory: {}", err))),
            )
                .into_response();
        }
    };

    let breadcrumbs = format_breadcrumbs(&relative);
    (StatusCode::OK, Html(ui::render_browse_page(&breadcrumbs, &folders, &videos))).into_response()
}

async fn video_thumbnail(State(state): State<AppState>, Path(path): Path<String>) -> Response {
    let (_absolute, relative) = match resolve_video_file(state.media_root.as_ref(), &path) {
        Ok(values) => values,
        Err(err) => return render_path_error(err),
    };

    let thumbnail_path = match valid_thumbnail_path_for_video(state.media_root.as_ref(), &relative) {
        Some(path) => path,
        None => return (StatusCode::NOT_FOUND, "Thumbnail not found").into_response(),
    };

    match std::fs::read(thumbnail_path) {
        Ok(bytes) => (
            StatusCode::OK,
            [(axum::http::header::CONTENT_TYPE, "image/png")],
            bytes,
        )
            .into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Could not read thumbnail").into_response(),
    }
}

async fn play(State(state): State<AppState>, Path(path): Path<String>) -> Response {
    let (_absolute, relative) = match resolve_video_file(state.media_root.as_ref(), &path) {
        Ok(values) => values,
        Err(err) => return render_path_error(err),
    };

    let media_src = format!("/media/{}", encode_url_path(&relative));
    let parent_href = relative
        .parent()
        .map(|parent| format!("/browse/{}", encode_url_path(parent)))
        .unwrap_or_else(|| "/browse/".to_string());
    let display_name = relative
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| "Video".to_string());

    (
        StatusCode::OK,
        Html(ui::render_video_page(display_name, media_src, parent_href)),
    )
        .into_response()
}

fn render_path_error(err: ResolveError) -> Response {
    let (status, message) = match err {
        ResolveError::PathTraversal => (StatusCode::BAD_REQUEST, "Invalid path."),
        ResolveError::NotDirectory | ResolveError::NotFound | ResolveError::NotFile => {
            (StatusCode::NOT_FOUND, "Requested path was not found.")
        }
        ResolveError::UnsupportedVideo => (
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "Video format is not part of the native playback allow-list.",
        ),
    };

    (status, Html(ui::render_not_found(message.to_string()))).into_response()
}
