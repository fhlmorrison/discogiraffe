// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod songs;
mod utils;
mod ytdl;

use crate::utils::CommandError;

use tauri::{AppHandle, Manager};

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
async fn get_playlist_info(url: &str) -> Result<database::DbPlaylistFull, CommandError> {
    return ytdl::get_playlist_info(url);
}

#[tauri::command]
async fn add_playlist(handle: AppHandle, url: &str) -> Result<(), CommandError> {
    return handle.db(|conn| ytdl::add_to_db(conn, url));
}

#[tauri::command]
async fn load_playlist(
    handle: AppHandle,
    id: &str,
) -> Result<database::DbPlaylistFull, CommandError> {
    return handle.db(|conn| database::get_playlist(conn, id));
}

#[tauri::command]
async fn load_playlists(handle: AppHandle) -> Result<Vec<database::DbPlaylist>, CommandError> {
    return handle.db(|conn| database::get_playlists(conn));
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
fn write_metadata(
    app_handle: tauri::AppHandle,
    event: songs::WriteMetadataEvent,
) -> Result<(), CommandError> {
    app_handle.db(|conn| database::update_metadata(conn, &event))?;
    songs::write_metadata(event)?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(database::AppState {
            db: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_playlist_info,
            download_song,
            get_metadata,
            write_metadata,
            get_cover_art,
            add_playlist,
            load_playlists,
            load_playlist
        ])
        .setup(|app| {
            let handle = app.handle();

            let app_state: tauri::State<database::AppState> = handle.state();
            let db = database::init_db(&handle).expect("failed to open database");
            *app_state.db.lock().unwrap() = Some(db);

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

trait DatabaseAccess {
    fn db<F, T>(&self, operation: F) -> T
    where
        F: FnOnce(&rusqlite::Connection) -> T;
}

impl DatabaseAccess for tauri::AppHandle {
    fn db<F, T>(&self, operation: F) -> T
    where
        F: FnOnce(&rusqlite::Connection) -> T,
    {
        let state = self.state::<database::AppState>();
        let db_guard = state.db.lock().unwrap();
        let db = db_guard.as_ref().unwrap();
        operation(db)
    }
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
