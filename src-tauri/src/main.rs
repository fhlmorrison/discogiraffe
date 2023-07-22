// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod songs;
mod utils;

use crate::utils::CommandError;

use tauri::Manager;

use std::path::PathBuf;
use ytd_rs::{Arg, YoutubeDL};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    println!("Rust says hello to {}!", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn download_song(url: &str, download_path: &str) -> Result<String, CommandError> {
    let args = vec![
        Arg::new("-icwx"),
        Arg::new_with_arg("-f", "bestaudio/best"),
        Arg::new_with_arg("-o", "%(title)s-%(id)s.%(ext)s"),
        Arg::new_with_arg("--audio-format", "mp3"),
        Arg::new_with_arg("--audio-quality", "0"),
        Arg::new("--embed-thumbnail"),
    ];
    let path = PathBuf::from(download_path);

    print!("yt-dlp");
    args.iter().for_each(|arg| print!(" {}", arg.to_string()));
    print!(" {}\n", url);

    let ytd = YoutubeDL::new(&path, args, url)?;

    println!("Downloading song from {} to {}", url, path.display());
    let download = ytd.download()?;

    let filename = parse_filename(download.output());
    let filepath = path.join(filename);
    println!("Downloaded song to {}", filepath.display());
    return Ok(filepath.display().to_string());
}

#[tauri::command]
async fn get_playlist_info(url: &str) -> Result<String, CommandError> {
    let args = vec![
        Arg::new("-J"),
        Arg::new("--skip-download"),
        Arg::new("--flat-playlist"),
    ];
    let link = url;
    let path = PathBuf::from("./");

    let ytd = YoutubeDL::new(&path, args, link)?;

    println!("Downloading playlist info for {}", url);
    let download = ytd.download()?;

    // println!("{}", download.output().to_string());

    return Ok(download.output().to_string());
}

#[tauri::command]
fn get_metadata(src: &str) -> Vec<songs::MetadataEntry> {
    return songs::read_metadata(src);
}

#[tauri::command]
fn get_cover_art(src: &str) -> Result<songs::CoverArt, CommandError> {
    return songs::read_cover_art(src);
}

#[tauri::command]
fn write_metadata(event: songs::WriteMetadataEvent) -> Result<(), CommandError> {
    return songs::write_metadata(event);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_playlist_info,
            download_song,
            get_metadata,
            write_metadata,
            get_cover_art
        ])
        .setup(|app| {
            let handle = app.handle();
            let db = database::init_db(&handle).unwrap();
            let database_lock = std::sync::Mutex::new(Some(db));
            let app_state = database::AppState { db: database_lock };
            handle.manage(app_state);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn parse_filename(input: &str) -> String {
    return input
        .trim_end()
        .trim_end_matches("\"")
        .split("\"")
        .last()
        .unwrap()
        .to_string();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_filename() {
        let test = r#"[youtube] Extracting URL: https://www.youtube.com/watch?v=YERGdRtm5q0
[youtube] YERGdRtm5q0: Downloading webpage
[youtube] YERGdRtm5q0: Downloading android player API JSON
[info] YERGdRtm5q0: Downloading 1 format(s): 251
[info] Downloading video thumbnail 41 ...
[info] Writing video thumbnail 41 to: jamesjamesjames - Glamorous-YERGdRtm5q0.webp
[dashsegments] Total fragments: 1
[download] Destination: jamesjamesjames - Glamorous-YERGdRtm5q0.webm
[download] 100% of    4.45MiB in 00:00:01 at 4.33MiB/s
[ExtractAudio] Destination: jamesjamesjames - Glamorous-YERGdRtm5q0.mp3
Deleting original file jamesjamesjames - Glamorous-YERGdRtm5q0.webm (pass -k to keep)
[ThumbnailsConvertor] Converting thumbnail "jamesjamesjames - Glamorous-YERGdRtm5q0.webp" to png
[EmbedThumbnail] ffmpeg: Adding thumbnail to "jamesjamesjames - Glamorous-YERGdRtm5q0.mp3"

"#;

        let expected = "jamesjamesjames - Glamorous-YERGdRtm5q0.mp3".to_string();
        let actual = super::parse_filename(test);
        println!("{}", actual);
        assert_eq!(expected, actual);
    }
}
