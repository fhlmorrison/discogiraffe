// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::{engine::general_purpose, Engine as _};
use id3::{Tag, TagLike};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;
use thiserror::Error;
use ytd_rs::{Arg, YoutubeDL};

#[derive(Debug, Error)]
enum CommandError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    YoutubeDL(#[from] ytd_rs::error::YoutubeDLError),

    #[error(transparent)]
    Id3(#[from] id3::Error),
}

// impl std::error::Error for CommandError {
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//         match self {
//             CommandError::Io(err) => Some(err),
//             CommandError::YoutubeDL(err) => Some(err),
//             CommandError::Id3(err) => Some(err),
//         }
//     }
// }

// we must manually implement serde::Serialize
impl serde::Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

use std::fmt;

pub fn iterable_to_str<I, D>(iterable: I) -> String
where
    I: IntoIterator<Item = D>,
    D: fmt::Display,
{
    let mut iterator = iterable.into_iter();

    let head = match iterator.next() {
        None => return String::from("[]"),
        Some(x) => format!("[{}", x),
    };
    let body = iterator.fold(head, |a, v| format!("{}, {}", a, v));
    format!("{}]", body)
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    println!("Rust says hello to {}!", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn download_song(url: &str, download_path: &str) -> Result<String, CommandError> {
    let args = vec![
        Arg::new("-icwx"),
        Arg::new_with_arg("-f", "bestaudio/best"),
        Arg::new_with_arg("-o", "%(title)s-%(id)s.%(ext)s"),
        Arg::new_with_arg("--audio-format", "mp3"),
        Arg::new_with_arg("--audio-quality", "0"),
        Arg::new("--embed-thumbnail"),
    ];
    let link = url;
    let path = PathBuf::from(download_path);
    args.iter().for_each(|arg| println!("{}", arg.to_string()));

    let ytd = YoutubeDL::new(&path, args, link)?;

    println!("Downloading song from {} to {}", url, path.display());
    let download = ytd.download()?;

    println!("{}", download.output().to_string());
    println!("{}", download.output_dir().display());
    return Ok(download.output_dir().to_string_lossy().to_string());
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

#[derive(Serialize, Deserialize)]
enum MetadataKey {
    Title,
    Artist,
    Album,
    AudioSourceUrl,
}

#[derive(Serialize, Deserialize)]
struct MetadataEntry {
    key: MetadataKey,
    value: Value,
}

#[derive(Serialize, Deserialize)]
struct WriteMetadataEvent {
    src: String,
    metadata: Vec<MetadataEntry>,
}

#[tauri::command]
fn get_metadata(src: &str) -> Vec<MetadataEntry> {
    let path = PathBuf::from(src);
    let tag = Tag::read_from_path(path).unwrap_or_default();
    let title = tag.title().map(|s| s.to_string());
    let artist = tag.artist().map(|s| s.to_string());
    let album = tag.album().map(|s| s.to_string());
    // let audio_source_url1 = tag.get("WOAS");
    // println!("{:?}", audio_source_url1);
    let audio_source_url = tag.get("WOAS").and_then(|frame| {
        frame
            .content()
            .link()
            .map(|s| s.trim_matches('"').to_string())
    });

    let metadata = vec![
        MetadataEntry {
            key: MetadataKey::Title,
            value: Value::String(title.unwrap_or_default()),
        },
        MetadataEntry {
            key: MetadataKey::Artist,
            value: Value::String(artist.unwrap_or_default()),
        },
        MetadataEntry {
            key: MetadataKey::Album,
            value: Value::String(album.unwrap_or_default()),
        },
        MetadataEntry {
            key: MetadataKey::AudioSourceUrl,
            value: Value::String(audio_source_url.unwrap_or_default()),
        },
    ];
    return metadata;
}

#[derive(Serialize, Deserialize)]
struct CoverArt {
    mime_type: String,
    b64: String,
}

#[tauri::command]
fn get_cover_art(src: &str) -> Result<CoverArt, CommandError> {
    let path = PathBuf::from(src);
    let tag = Tag::read_from_path(path).unwrap_or_default();

    let cover_art = tag.pictures().next();
    let mime_type = cover_art
        .map(|p| &p.mime_type)
        .unwrap_or(&"".to_string())
        .to_string();
    let picture_data = cover_art.map(|p| p.data.to_vec());

    let b64 = general_purpose::STANDARD.encode(&picture_data.unwrap_or_default());
    return Ok(CoverArt { mime_type, b64 });
}

#[tauri::command]
fn write_metadata(event: WriteMetadataEvent) -> Result<(), CommandError> {
    println!("Writing metadata to {}", event.src);
    // let payload = event.payload().unwrap();
    // let event: WriteMetadataEvent = serde_json::from_str(payload).unwrap();
    if event.metadata.is_empty() {
        return Ok(());
    }
    let path = PathBuf::from(event.src);
    let mut tag = match Tag::read_from_path(&path) {
        Ok(tag) => tag,
        Err(_) => Tag::new(),
    };
    for entry in event.metadata.iter() {
        match entry.key {
            MetadataKey::Title => {
                let title = entry.value.as_str().unwrap();
                tag.set_title(title);
            }
            MetadataKey::Artist => {
                let artist = entry.value.as_str().unwrap();
                tag.set_artist(artist);
            }
            MetadataKey::Album => {
                let album = entry.value.as_str().unwrap();
                tag.set_album(album);
            }
            MetadataKey::AudioSourceUrl => {
                let audio_source_url = entry.value.to_string();
                tag.add_frame(id3::Frame::link("WAS", audio_source_url));
            }
            _ => {}
        }
    }
    tag.write_to_path(path, id3::Version::Id3v24)?;

    Ok(())
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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
