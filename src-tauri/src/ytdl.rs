use crate::database::{DbPlaylist, DbSong};
use crate::utils::CommandError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ytd_rs::{Arg, YoutubeDL};

pub async fn get_playlist_info(url: &str) -> Result<String, CommandError> {
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

    return Ok(download.output().to_owned());
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
}

#[derive(Serialize, Deserialize)]
pub struct YTPlaylist {
    pub id: String,
    pub title: String,
    pub description: String,
    pub webpage_url: String,
    pub thumbnails: Vec<YTThumbnail>,
    pub channel: Option<String>,
    pub entries: Vec<YTSong>,
}

pub async fn parse_playlist(url: &str) -> Result<YTPlaylist, CommandError> {
    let data = get_playlist_info(url).await?;
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

pub async fn add_to_db(url: &str) -> Result<(), CommandError> {
    let playlist_info = parse_playlist(url).await?;
    let playlist = cast_playlist_info(&playlist_info);
    let songs: Vec<DbSong> = playlist_info
        .entries
        .iter()
        .map(|song| cast_song(song))
        .collect();

    Ok(())
}

mod tests {
    use super::parse_playlist;
    use tauri::async_runtime::block_on;

    #[test]
    fn test_parse_playlist() {
        let data = block_on(parse_playlist(
            "https://www.youtube.com/playlist?list=PLEfmir4ilkztwpqgkqKBle-i86tN_wOsJ",
        ));
        match data {
            Ok(val) => println!("{}", val.title),
            Err(e) => println!("{}", e),
        }
    }
}
