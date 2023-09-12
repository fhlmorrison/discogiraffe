// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod songs;
mod utils;
mod ytdl;

use crate::{
    songs::{MetadataEntry, MetadataKey},
    utils::CommandError,
};

use rusqlite::Connection;
use songs::WriteMetadataEvent;
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
async fn download_song(
    handle: AppHandle,
    // url: &str,
    song: database::DbSong,
    download_path: &str,
) -> Result<String, CommandError> {
    let url = song.url.as_str();
    let channel = song.channel.unwrap_or("".to_string());
    // return ytdl::download_song(url, download_path);

    match ytdl::download_song(url, download_path) {
        Ok(path) => {
            match handle.db(|db| database::add_local_song(db, &path, &url)) {
                Ok(_) => {
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                    write_metadata(
                        handle,
                        WriteMetadataEvent {
                            src: path.clone(),
                            metadata: vec![
                                MetadataEntry {
                                    key: MetadataKey::AudioSourceUrl,
                                    value: serde_json::Value::String(url.to_string()),
                                },
                                MetadataEntry {
                                    key: MetadataKey::Album,
                                    value: serde_json::Value::String(channel.to_string()),
                                },
                            ],
                        },
                    )
                    .await?;
                    println!("{}", "set as downloaded")
                }
                Err(e) => {
                    print!("{}", e)
                }
            };
            return Ok(path);
        }
        Err(e) => {
            return Err(e);
        }
    }
}

#[tauri::command]
async fn save_folder(app: tauri::AppHandle, files: Vec<&str>) -> Result<(), CommandError> {
    // TODO: needs to be improved so errors bubble up
    println!("Checking {} files against database", files.len());
    files
        .iter()
        .map(|&file| {
            return save_local_song(&app, file);
        })
        .collect::<Result<_, CommandError>>()?;
    println!("Finished saving files to database");
    Ok(())
}

fn save_local_song(app: &tauri::AppHandle, file: &str) -> Result<(), CommandError> {
    let metadata = songs::read_metadata(file);
    if let Some(Some(url)) = metadata
        .iter()
        .find(|x| matches!(x.key, songs::MetadataKey::AudioSourceUrl))
        .map(|x| x.value.as_str())
    {
        println!("Saving {} to database", file);
        app.db(|conn| database::add_local_song(conn, file, url))?;
        app.db(|conn| {
            database::update_metadata(
                conn,
                &WriteMetadataEvent {
                    src: file.to_string(),
                    metadata: metadata,
                },
            )
        })?;
    } else {
        println!("No audio source url found for {}", file);
    };
    Ok(())
}

#[tauri::command]
async fn get_playlist_info(url: &str) -> Result<database::DbPlaylistFull, CommandError> {
    return ytdl::get_playlist_info(url);
}

#[tauri::command]
async fn add_playlist(handle: AppHandle, url: &str) -> Result<(), CommandError> {
    let playlist = ytdl::get_playlist_info(url)?;
    return handle.db(|conn| database::add_playlist(conn, playlist));
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
async fn write_metadata(
    app_handle: tauri::AppHandle,
    event: songs::WriteMetadataEvent,
) -> Result<(), CommandError> {
    app_handle.db(|conn| database::update_metadata(conn, &event))?;
    songs::write_metadata(event).await?;
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
            load_playlist,
            save_folder
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
