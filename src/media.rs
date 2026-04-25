use std::path::{Component, Path, PathBuf};

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

use crate::thumbnail;

#[derive(Debug, Clone)]
pub struct FolderEntry {
    pub name: String,
    pub relative_path: PathBuf,
    pub thumbnail_relative_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct VideoEntry {
    pub name: String,
    pub relative_path: PathBuf,
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SearchEntry {
    pub relative_path: PathBuf,
    pub kind: SearchEntryKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchEntryKind {
    Folder,
    Video,
}

#[derive(Debug)]
pub enum ResolveError {
    PathTraversal,
    NotDirectory,
    NotFound,
    NotFile,
    UnsupportedVideo,
}

const SUPPORTED_VIDEO_EXTENSIONS: &[&str] = &["mp4", "webm", "mkv", "mov", "avi"];

pub fn normalize_relative_path(raw: &str) -> Result<PathBuf, ResolveError> {
    let mut clean = PathBuf::new();
    let trimmed = raw.trim_matches('/');

    if trimmed.is_empty() {
        return Ok(clean);
    }

    for component in Path::new(trimmed).components() {
        match component {
            Component::Normal(part) => clean.push(part),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(ResolveError::PathTraversal)
            }
        }
    }

    Ok(clean)
}

pub fn resolve_directory(root: &Path, relative_raw: &str) -> Result<(PathBuf, PathBuf), ResolveError> {
    let relative = normalize_relative_path(relative_raw)?;
    let absolute = root.join(&relative);

    if !absolute.exists() {
        return Err(ResolveError::NotFound);
    }

    if !absolute.is_dir() {
        return Err(ResolveError::NotDirectory);
    }

    Ok((absolute, relative))
}

pub fn resolve_video_file(root: &Path, relative_raw: &str) -> Result<(PathBuf, PathBuf), ResolveError> {
    let relative = normalize_relative_path(relative_raw)?;
    let absolute = root.join(&relative);

    if !absolute.exists() {
        return Err(ResolveError::NotFound);
    }

    if !absolute.is_file() {
        return Err(ResolveError::NotFile);
    }

    if !is_supported_video_file(&absolute) {
        return Err(ResolveError::UnsupportedVideo);
    }

    Ok((absolute, relative))
}

pub fn scan_directory(
    media_root: &Path,
    absolute_dir: &Path,
    relative_dir: &Path,
) -> std::io::Result<(Vec<FolderEntry>, Vec<VideoEntry>)> {
    let mut folders = Vec::new();
    let mut videos = Vec::new();

    for entry_result in std::fs::read_dir(absolute_dir)? {
        let entry = entry_result?;
        let path = entry.path();
        let file_name = entry.file_name().to_string_lossy().to_string();

        if path.is_dir() {
            if directory_is_empty(&path)? {
                continue;
            }

            let folder_relative_path = relative_dir.join(&file_name);
            let thumbnail = path.join("folder.jpg");
            let thumbnail_relative_path = if thumbnail.is_file() {
                Some(folder_relative_path.join("folder.jpg"))
            } else {
                None
            };

            folders.push(FolderEntry {
                name: file_name,
                relative_path: folder_relative_path,
                thumbnail_relative_path,
            });
        } else if path.is_file() && is_supported_video_file(&path) {
            let video_relative_path = relative_dir.join(entry.file_name());
            let thumbnail_url = if thumbnail::valid_thumbnail_path_for_video(media_root, &video_relative_path).is_some() {
                Some(format!("/thumb/video/{}", encode_url_path(&video_relative_path)))
            } else {
                None
            };

            videos.push(VideoEntry {
                name: file_name,
                relative_path: video_relative_path,
                thumbnail_url,
            });
        }
    }

    folders.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    videos.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    Ok((folders, videos))
}

pub fn search_paths(media_root: &Path, query: &str) -> std::io::Result<Vec<SearchEntry>> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    let query_lower = trimmed.to_lowercase();
    let mut matches = Vec::new();
    let mut root_relative = PathBuf::new();
    search_paths_inner(media_root, media_root, &mut root_relative, &query_lower, &mut matches)?;

    matches.sort_by(|a, b| {
        let left = a.relative_path.to_string_lossy().to_lowercase();
        let right = b.relative_path.to_string_lossy().to_lowercase();
        left.cmp(&right)
    });

    Ok(matches)
}

fn search_paths_inner(
    media_root: &Path,
    absolute_dir: &Path,
    relative_dir: &mut PathBuf,
    query_lower: &str,
    matches: &mut Vec<SearchEntry>,
) -> std::io::Result<()> {
    for entry_result in std::fs::read_dir(absolute_dir)? {
        let entry = entry_result?;
        let file_name = entry.file_name();
        let path = entry.path();

        relative_dir.push(&file_name);
        let relative_path = relative_dir.clone();
        let relative_text = relative_path.to_string_lossy().to_lowercase();

        if path.is_dir() {
            if !directory_is_empty(&path)? && relative_text.contains(query_lower) {
                matches.push(SearchEntry {
                    relative_path: relative_path.clone(),
                    kind: SearchEntryKind::Folder,
                });
            }

            // Recurse through all folders to support searching the entire media root.
            if let Err(err) = search_paths_inner(media_root, &path, relative_dir, query_lower, matches) {
                relative_dir.pop();
                return Err(err);
            }
        } else if path.is_file() && is_supported_video_file(&path) {
            if relative_text.contains(query_lower) {
                matches.push(SearchEntry {
                    relative_path,
                    kind: SearchEntryKind::Video,
                });
            }
        }

        relative_dir.pop();
    }

    let _ = media_root;
    Ok(())
}

pub fn encode_url_path(path: &Path) -> String {
    path.iter()
        .map(|segment| utf8_percent_encode(&segment.to_string_lossy(), NON_ALPHANUMERIC).to_string())
        .collect::<Vec<_>>()
        .join("/")
}

pub fn format_breadcrumbs(path: &Path) -> Vec<(String, String)> {
    let mut crumbs = vec![("Home".to_string(), "/browse/".to_string())];
    let mut running = PathBuf::new();

    for segment in path.iter() {
        running.push(segment);
        let segment_text = segment.to_string_lossy().to_string();
        let href = format!("/browse/{}", encode_url_path(&running));
        crumbs.push((segment_text, href));
    }

    crumbs
}

fn is_supported_video_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| SUPPORTED_VIDEO_EXTENSIONS.contains(&ext.to_ascii_lowercase().as_str()))
        .unwrap_or(false)
}

fn directory_is_empty(path: &Path) -> std::io::Result<bool> {
    Ok(std::fs::read_dir(path)?.next().is_none())
}
