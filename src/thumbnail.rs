use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use url::Url;

const SIZE_DIRS: &[&str] = &["normal", "large", "x-large", "xx-large"];

pub fn valid_thumbnail_path_for_video(media_root: &Path, relative_video_path: &Path) -> Option<PathBuf> {
    let absolute_video_path = media_root.join(relative_video_path);
    let video_mtime = file_mtime_secs(&absolute_video_path).ok()?;
    let uri_variants = uri_variants_for_path(&absolute_video_path);

    for video_uri in &uri_variants {
        let hash = format!("{:x}", md5::compute(video_uri.as_bytes()));

        for dir in SIZE_DIRS {
            let candidate = gnome_thumbnail_base_dir().join(dir).join(format!("{hash}.png"));
            if candidate.is_file() && thumbnail_png_is_valid(&candidate, video_uri, video_mtime) {
                return Some(candidate);
            }
        }
    }

    None
}

fn gnome_thumbnail_base_dir() -> PathBuf {
    if let Ok(cache) = std::env::var("XDG_CACHE_HOME") {
        return PathBuf::from(cache).join("thumbnails");
    }

    if let Ok(home) = std::env::var("HOME") {
        return PathBuf::from(home).join(".cache").join("thumbnails");
    }

    PathBuf::from(".cache").join("thumbnails")
}

fn file_uri(path: &Path) -> Option<String> {
    Url::from_file_path(path).ok().map(|uri| uri.to_string())
}

fn uri_variants_for_path(path: &Path) -> Vec<String> {
    let mut variants = Vec::new();

    if let Some(uri) = file_uri(path) {
        variants.push(uri);
    }

    if let Ok(canonical) = std::fs::canonicalize(path) {
        if let Some(uri) = file_uri(&canonical) {
            if !variants.iter().any(|v| v == &uri) {
                variants.push(uri);
            }
        }
    }

    variants
}

fn file_mtime_secs(path: &Path) -> std::io::Result<u64> {
    let modified = std::fs::metadata(path)?.modified()?;
    Ok(modified
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs())
}

fn thumbnail_png_is_valid(path: &Path, expected_uri: &str, expected_mtime: u64) -> bool {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return false,
    };

    let decoder = png::Decoder::new(BufReader::new(file));
    let reader = match decoder.read_info() {
        Ok(reader) => reader,
        Err(_) => return false,
    };

    let mut found_uri = None;
    let mut found_mtime = None;

    for chunk in &reader.info().uncompressed_latin1_text {
        collect_thumbnail_metadata(
            &chunk.keyword,
            &chunk.text,
            &mut found_uri,
            &mut found_mtime,
        );
    }

    for chunk in &reader.info().compressed_latin1_text {
        if let Ok(text) = chunk.get_text() {
            collect_thumbnail_metadata(
                &chunk.keyword,
                &text,
                &mut found_uri,
                &mut found_mtime,
            );
        }
    }

    for chunk in &reader.info().utf8_text {
        if let Ok(text) = chunk.get_text() {
            collect_thumbnail_metadata(
                &chunk.keyword,
                &text,
                &mut found_uri,
                &mut found_mtime,
            );
        }
    }

    let uri_ok = found_uri
        .as_deref()
        .map(|uri| uri == expected_uri)
        .unwrap_or(false);
    let mtime_ok = found_mtime
        .map(|mtime| mtime == expected_mtime)
        .unwrap_or(true);

    uri_ok && mtime_ok
}

fn collect_thumbnail_metadata(
    key: &str,
    value: &str,
    found_uri: &mut Option<String>,
    found_mtime: &mut Option<u64>,
) {
    if key == "Thumb::URI" {
        *found_uri = Some(value.to_string());
    }

    if key == "Thumb::MTime" {
        if let Ok(stored) = value.parse::<u64>() {
            *found_mtime = Some(stored);
        }
    }
}
