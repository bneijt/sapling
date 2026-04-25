use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use url::Url;

const SIZE_DIRS: &[&str] = &["normal", "large", "x-large", "xx-large"];
const GENERATED_SIZE: u32 = 256;
const FAIL_APP_NAME: &str = "gnome-thumbnail-factory";
const THUMB_SOFTWARE: &str = "Sapling";

pub fn get_or_generate_thumbnail_path_for_video(media_root: &Path, relative_video_path: &Path) -> Option<PathBuf> {
    if let Some(path) = valid_thumbnail_path_for_video(media_root, relative_video_path) {
        return Some(path);
    }

    let absolute_video_path = media_root.join(relative_video_path);
    let video_mtime = file_mtime_secs(&absolute_video_path).ok()?;
    let uris = uri_variants_for_path(&absolute_video_path);
    let primary_uri = uris.first()?.clone();

    if has_valid_failed_thumbnail(&uris, video_mtime) {
        return None;
    }

    match generate_thumbnail_for_video(&absolute_video_path, &primary_uri, video_mtime) {
        Ok(path) => Some(path),
        Err(_) => {
            let _ = write_failed_thumbnail_marker(&primary_uri, video_mtime);
            None
        }
    }
}

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

fn generate_thumbnail_for_video(video_path: &Path, uri: &str, mtime: u64) -> std::io::Result<PathBuf> {
    let output_path = thumbnail_cache_path(uri, "normal");
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let temp_output = temp_thumbnail_path("sapling-thumb");
    if let Some(parent) = temp_output.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let generated = run_thumbnailers(video_path, uri, &temp_output);

    if generated && temp_output.is_file() {
        write_png_with_metadata(&temp_output, &output_path, uri, mtime)?;
        let _ = std::fs::remove_file(&temp_output);
        return Ok(output_path);
    }

    let _ = std::fs::remove_file(&temp_output);
    Err(std::io::Error::other("thumbnail generation failed"))
}

fn run_thumbnailers(video_path: &Path, uri: &str, output: &Path) -> bool {
    if run_totem_thumbnailer(uri, output) {
        return true;
    }

    run_ffmpeg_thumbnailer(video_path, output)
}

fn run_totem_thumbnailer(uri: &str, output: &Path) -> bool {
    if !command_available("totem-video-thumbnailer") {
        return false;
    }

    Command::new("totem-video-thumbnailer")
        .arg("-s")
        .arg(GENERATED_SIZE.to_string())
        .arg(uri)
        .arg(output)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn run_ffmpeg_thumbnailer(video_path: &Path, output: &Path) -> bool {
    if !command_available("ffmpegthumbnailer") {
        return false;
    }

    Command::new("ffmpegthumbnailer")
        .arg("-i")
        .arg(video_path)
        .arg("-o")
        .arg(output)
        .arg("-s")
        .arg(GENERATED_SIZE.to_string())
        .arg("-t")
        .arg("10%")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn command_available(name: &str) -> bool {
    Command::new(name)
        .arg("--help")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn has_valid_failed_thumbnail(uris: &[String], mtime: u64) -> bool {
    for uri in uris {
        let candidate = failed_thumbnail_path(uri);
        if candidate.is_file() && thumbnail_png_is_valid(&candidate, uri, mtime) {
            return true;
        }
    }

    false
}

fn write_failed_thumbnail_marker(uri: &str, mtime: u64) -> std::io::Result<()> {
    let path = failed_thumbnail_path(uri);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    write_marker_png(&path, uri, mtime)
}

fn failed_thumbnail_path(uri: &str) -> PathBuf {
    let hash = format!("{:x}", md5::compute(uri.as_bytes()));
    gnome_thumbnail_base_dir()
        .join("fail")
        .join(FAIL_APP_NAME)
        .join(format!("{hash}.png"))
}

fn thumbnail_cache_path(uri: &str, size_dir: &str) -> PathBuf {
    let hash = format!("{:x}", md5::compute(uri.as_bytes()));
    gnome_thumbnail_base_dir().join(size_dir).join(format!("{hash}.png"))
}

fn temp_thumbnail_path(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}-{nanos}.png"))
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

fn write_png_with_metadata(input_path: &Path, output_path: &Path, uri: &str, mtime: u64) -> std::io::Result<()> {
    let file = File::open(input_path)?;
    let decoder = png::Decoder::new(BufReader::new(file));
    let mut reader = decoder
        .read_info()
        .map_err(|err| std::io::Error::other(err.to_string()))?;
    let mut buffer = vec![0; reader.output_buffer_size()];
    let frame = reader
        .next_frame(&mut buffer)
        .map_err(|err| std::io::Error::other(err.to_string()))?;
    let image_data = &buffer[..frame.buffer_size()];

    let out = File::create(output_path)?;
    let mut encoder = png::Encoder::new(out, frame.width, frame.height);
    encoder.set_color(frame.color_type);
    encoder.set_depth(frame.bit_depth);
    encoder
        .add_text_chunk("Thumb::URI".to_string(), uri.to_string())
        .map_err(|err| std::io::Error::other(err.to_string()))?;
    encoder
        .add_text_chunk("Thumb::MTime".to_string(), mtime.to_string())
        .map_err(|err| std::io::Error::other(err.to_string()))?;
    encoder
        .add_text_chunk("Thumb::Software".to_string(), THUMB_SOFTWARE.to_string())
        .map_err(|err| std::io::Error::other(err.to_string()))?;

    let mut writer = encoder
        .write_header()
        .map_err(|err| std::io::Error::other(err.to_string()))?;
    writer
        .write_image_data(image_data)
        .map_err(|err| std::io::Error::other(err.to_string()))
}

fn write_marker_png(path: &Path, uri: &str, mtime: u64) -> std::io::Result<()> {
    let out = File::create(path)?;
    let mut encoder = png::Encoder::new(out, 1, 1);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder
        .add_text_chunk("Thumb::URI".to_string(), uri.to_string())
        .map_err(|err| std::io::Error::other(err.to_string()))?;
    encoder
        .add_text_chunk("Thumb::MTime".to_string(), mtime.to_string())
        .map_err(|err| std::io::Error::other(err.to_string()))?;
    encoder
        .add_text_chunk("Thumb::Software".to_string(), THUMB_SOFTWARE.to_string())
        .map_err(|err| std::io::Error::other(err.to_string()))?;

    let mut writer = encoder
        .write_header()
        .map_err(|err| std::io::Error::other(err.to_string()))?;
    writer
        .write_image_data(&[0, 0, 0, 0])
        .map_err(|err| std::io::Error::other(err.to_string()))
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
