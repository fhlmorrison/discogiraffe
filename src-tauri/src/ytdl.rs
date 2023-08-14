use crate::database::{DbPlaylist, DbPlaylistFull, DbSong};
use crate::utils::CommandError;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ytd_rs::{Arg, YoutubeDL};

pub fn fetch_playlist(url: &str) -> Result<String, CommandError> {
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
    pub duration: f32,
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

fn parse_playlist(url: &str) -> Result<YTPlaylist, CommandError> {
    let data = fetch_playlist(url)?;
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

pub fn get_playlist_info(url: &str) -> Result<DbPlaylistFull, CommandError> {
    let playlist_info = parse_playlist(url)?;
    let playlist = cast_playlist_info(&playlist_info);
    let songs: Vec<DbSong> = playlist_info
        .entries
        .iter()
        .map(|song| cast_song(song))
        .collect();

    let result = DbPlaylistFull { playlist, songs };

    Ok(result)
}

pub fn add_to_db<'a>(db: &Connection, url: &str) -> Result<(), CommandError> {
    let playlist_info = parse_playlist(url)?;
    let playlist = cast_playlist_info(&playlist_info);
    let songs: Vec<DbSong> = playlist_info
        .entries
        .iter()
        .map(|song| cast_song(song))
        .collect();

    let tx = db.unchecked_transaction()?;

    tx.execute(
        "INSERT INTO playlists (id, title, description, url, thumbnail, path, downloaded) 
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7) ON CONFLICT(id) DO UPDATE 
        SET title = ?2, description = ?3, url = ?4, thumbnail = ?5, path = ?6, downloaded = ?7",
        params![
            playlist.id,
            playlist.title,
            playlist.description,
            playlist.url,
            playlist.thumbnail,
            playlist.path,
            playlist.downloaded
        ],
    )?;

    // TODO: replace unwraps

    songs.iter().for_each(|song| {
        tx.execute(
            "INSERT INTO songs 
            (id, title, url, thumbnail, path, downloaded, artist, album, audio_source_url, channel, duration) 
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            ON CONFLICT DO NOTHING
            ",
            params![
                song.id,
                song.title,
                song.url,
                song.thumbnail,
                song.path,
                song.downloaded,
                song.artist,
                song.album,
                song.audio_source_url,
                song.channel,
                song.duration
            ],
        )
        .unwrap();
        tx.execute(
            "INSERT INTO playlist_songs (playlist_id, song_id) VALUES (?1, ?2) ON CONFLICT DO NOTHING",
            params![playlist.id.to_owned(), song.id.to_owned()],
        )
        .unwrap();
    });

    tx.commit()?;

    Ok(())
}

mod tests {
    use super::parse_playlist;
    use tauri::async_runtime::block_on;

    #[test]
    fn test_parse_playlist() {
        let data = parse_playlist(
            "https://www.youtube.com/playlist?list=PLEfmir4ilkztwpqgkqKBle-i86tN_wOsJ",
        );
        match data {
            Ok(val) => println!("{}", val.title),
            Err(e) => println!("{}", e),
        }
    }
}
