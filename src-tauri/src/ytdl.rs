use crate::database::{DbPlaylist, DbPlaylistFull, DbSong};
use crate::utils::{parse_filename, CommandError};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;

const YTDL_COMMAND: &str = "yt-dlp";

pub async fn fetch_playlist(handle: &AppHandle, url: &str) -> Result<String, CommandError> {
    let args = vec!["-J", "--skip-download", "--flat-playlist", url];

    let shell = handle.shell();

    println!("Downloading playlist info for {}", url);
    let output = shell.command(YTDL_COMMAND).args(args).output().await?;

    match output.status.success() {
        true => Ok(String::from_utf8(output.stdout).unwrap()),
        false => Err(CommandError::CustomError(
            String::from_utf8(output.stderr).unwrap(),
        )),
    }
}

#[derive(Serialize, Deserialize)]
pub struct YTThumbnail {
    pub height: i32,
    pub url: String,
    pub width: i32,
}

#[derive(Serialize, Deserialize)]
pub struct YTSong {
    pub id: String,
    pub title: String,
    pub url: String,
    pub thumbnails: Vec<YTThumbnail>,
    pub channel: Option<String>,
    pub duration: Option<f32>,
}

#[derive(Serialize, Deserialize)]
struct YTPlaylist {
    pub id: String,
    pub title: String,
    pub description: String,
    pub webpage_url: String,
    pub thumbnails: Vec<YTThumbnail>,
    pub channel: Option<String>,
    pub entries: Vec<YTSong>,
}

async fn parse_playlist(handle: &AppHandle, url: &str) -> Result<YTPlaylist, CommandError> {
    let data = fetch_playlist(handle, url).await?;
    let playlist: YTPlaylist = serde_json::from_str(&data)?;
    return Ok(playlist);
}

fn cast_song(song: &YTSong) -> DbSong {
    return DbSong {
        id: song.id.to_owned(),
        title: song.title.to_owned(),
        url: song.url.to_owned(),
        thumbnail: song.thumbnails.last().map(|item| item.url.to_owned()),
        path: None,
        downloaded: false,
        artist: None,
        album: None,
        audio_source_url: None,
        channel: song.channel.to_owned(),
        duration: song.duration,
    };
}

fn cast_playlist_info(pl: &YTPlaylist) -> DbPlaylist {
    DbPlaylist {
        id: pl.id.to_owned(),
        title: pl.title.to_owned(),
        description: pl.description.to_owned(),
        url: pl.webpage_url.to_owned(),
        thumbnail: pl.thumbnails.last().map(|item| item.url.to_owned()),
        path: None,
        downloaded: false,
    }
}

pub async fn get_playlist_info(
    handle: &AppHandle,
    url: &str,
) -> Result<DbPlaylistFull, CommandError> {
    let playlist_info = parse_playlist(handle, url).await?;
    let playlist = cast_playlist_info(&playlist_info);
    let songs: Vec<DbSong> = playlist_info
        .entries
        .iter()
        .map(|song| cast_song(song))
        .collect();

    let result = DbPlaylistFull { playlist, songs };

    Ok(result)
}

pub async fn download_song(
    handle: &AppHandle,
    url: &str,
    download_path: &str,
) -> Result<String, CommandError> {
    let args = vec![
        "-icwx",
        "-f",
        "bestaudio/best",
        "-o",
        "%(title)s-%(id)s.%(ext)s",
        "--audio-format",
        "mp3",
        "--audio-quality",
        "0",
        "--embed-thumbnail",
        url,
    ];
    let path = PathBuf::from(download_path);
    let cloned_path = path.clone();
    let shell = handle.shell();

    println!("Downloading song from {} to {}", url, path.display());

    let output = shell
        .command(YTDL_COMMAND)
        .current_dir(cloned_path)
        .args(args)
        .output()
        .await?;

    match output.status.success() {
        true => {
            let filename = parse_filename(String::from_utf8(output.stdout).unwrap().as_str());
            let filepath = path.join(filename);
            println!("Downloaded song to {}", filepath.display());
            Ok(filepath.display().to_string())
        }
        false => Err(CommandError::CustomError(
            String::from_utf8(output.stderr).unwrap(),
        )),
    }
}

// mod test_bin {
//     use crate::utils::{parse_filename, CommandError};
//     use std::path::PathBuf;
//     use tauri_plugin_process::ShellExt;

//     const YTDL_COMMAND: &str = "yt-dlp";

//     fn download_song(url: &str, download_path: &str) -> Result<String, CommandError> {
//         let args = vec![
//             "-icwx",
//             "-f",
//             "bestaudio/best",
//             "-o",
//             "%(title)s-%(id)s.%(ext)s",
//             "--audio-format",
//             "mp3",
//             "--audio-quality",
//             "0",
//             "--embed-thumbnail",
//             url,
//         ];
//         let path = PathBuf::from(download_path);

//         println!("Downloading song from {} to {}", url, path.display());

//         let output = Command::new(YTDL_COMMAND)
//             .current_dir(path.clone())
//             .args(args)
//             .output()?;

//         match output.status.success() {
//             true => {
//                 let filename = parse_filename(output.stdout.as_str());
//                 let filepath = path.join(filename);
//                 println!("Downloaded song to {}", filepath.display());
//                 return Ok(filepath.display().to_string());
//             }
//             false => return Err(CommandError::CustomError(output.stderr)),
//         };
//         Ok("".to_string())
//     }
// }

// mod tests {
//     use super::parse_playlist;
//     use tauri::async_runtime::block_on;

//     #[test]
//     fn test_parse_playlist() {
//         let data = parse_playlist(
//             "https://www.youtube.com/playlist?list=PLEfmir4ilkztwpqgkqKBle-i86tN_wOsJ",
//         );
//         match data {
//             Ok(val) => println!("{}", val.title),
//             Err(e) => println!("{}", e),
//         }
//     }
// }
