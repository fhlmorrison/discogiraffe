use rusqlite::{params, Connection, Result};
use tauri::{AppHandle, Manager};

use crate::songs::{MetadataKey, WriteMetadataEvent};
use crate::utils::CommandError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DbSong {
    pub id: String,
    pub title: String,
    pub url: String,
    pub thumbnail: Option<String>,
    pub path: Option<String>,
    pub downloaded: bool,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub audio_source_url: Option<String>,
    pub channel: Option<String>,
    pub duration: Option<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct DbPlaylist {
    pub id: String,
    pub title: String,
    pub description: String,
    pub url: String,
    pub thumbnail: Option<String>,
    pub path: Option<String>,
    pub downloaded: bool,
}

#[derive(Serialize, Deserialize)]
pub struct DbPlaylistSong {
    pub id: i32,
    pub playlist_id: i32,
    pub song_id: i32,
}

#[derive(Serialize, Deserialize)]

pub struct DbPlaylistFull {
    pub playlist: DbPlaylist,
    pub songs: Vec<DbSong>,
}

pub struct AppState {
    pub db: std::sync::Mutex<Option<Connection>>,
}

pub fn init_db(handle: &AppHandle) -> Result<Connection> {
    let dir = handle
        .path()
        .app_data_dir()
        .expect("failed to get config dir");

    if !dir.is_dir() {
        std::fs::create_dir_all(dir.clone()).map_err(|e| {
            rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(1),
                Some(format!("Failed to create directory: {}", e)),
            )
        })?;
    }

    let conn = Connection::open(dir.join("db.sqlite"))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS songs (
            id STRING PRIMARY KEY,
            title TEXT NOT NULL,
            url TEXT NOT NULL,
            thumbnail TEXT NOT NULL,
            path TEXT,
            downloaded BOOLEAN NOT NULL,
            artist TEXT,
            album TEXT,
            audio_source_url TEXT,
            channel TEXT,
            duration FLOAT
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS playlists (
            id STRING PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            url TEXT NOT NULL,
            thumbnail TEXT NOT NULL,
            path TEXT,
            downloaded BOOLEAN NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS playlist_songs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            playlist_id STRING NOT NULL,
            song_id STRING NOT NULL,
            UNIQUE (playlist_id, song_id),
            FOREIGN KEY (playlist_id) REFERENCES playlists(id) ON DELETE CASCADE ON UPDATE CASCADE,
            FOREIGN KEY (song_id) REFERENCES songs(id) ON DELETE CASCADE ON UPDATE CASCADE
        )",
        [],
    )?;

    Ok(conn)
}

// Add update metadata function

// Add

pub fn add_song(conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT INTO songs (title, url, thumbnail, path, downloaded) VALUES (?1, ?2, ?3, ?4, ?5)",
        [],
    )?;
    Ok(())
}

pub fn get_song(conn: &Connection, path: &str) -> Result<DbSong> {
    return conn.query_row("SELECT * FROM songs WHERE path=?1", [path], |row| {
        Ok(DbSong {
            id: row.get(0)?,
            title: row.get(1)?,
            url: row.get(2)?,
            thumbnail: row.get(3)?,
            path: row.get(4)?,
            downloaded: row.get(5)?,
            artist: row.get(6)?,
            album: row.get(7)?,
            audio_source_url: row.get(8)?,
            channel: row.get(9)?,
            duration: row.get(10)?,
        })
    });
}

pub fn get_playlists(conn: &Connection) -> Result<Vec<DbPlaylist>, CommandError> {
    let mut stmt = conn.prepare("SELECT * FROM playlists")?;

    let rows = stmt.query_map([], |row| {
        Ok(DbPlaylist {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            url: row.get(3)?,
            thumbnail: row.get(4)?,
            path: row.get(5)?,
            downloaded: row.get(6)?,
        })
    })?;

    let result = rows.into_iter().collect::<Result<Vec<_>, _>>()?;

    Ok(result)
}

pub fn get_playlist(conn: &Connection, id: &str) -> Result<DbPlaylistFull, CommandError> {
    let mut stmt = conn.prepare("SELECT * FROM playlists WHERE id=?1")?;
    let playlist = stmt.query_row([id], |row| {
        Ok(DbPlaylist {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            url: row.get(3)?,
            thumbnail: row.get(4)?,
            path: row.get(5)?,
            downloaded: row.get(6)?,
        })
    })?;

    let mut stmt = conn.prepare(
        "SELECT songs.* FROM songs
        INNER JOIN playlist_songs ON (songs.id = playlist_songs.song_id)
        WHERE playlist_songs.playlist_id = ?1
        ORDER BY playlist_songs.id;",
    )?;

    let rows = stmt.query_map([id], |row| {
        Ok(DbSong {
            id: row.get(0)?,
            title: row.get(1)?,
            url: row.get(2)?,
            thumbnail: row.get(3)?,
            path: row.get(4)?,
            downloaded: row.get(5)?,
            artist: row.get(6)?,
            album: row.get(7)?,
            audio_source_url: row.get(8)?,
            channel: row.get(9)?,
            duration: row.get(10)?,
        })
    })?;

    let songs = rows.into_iter().collect::<Result<Vec<_>, _>>()?;

    let result = DbPlaylistFull { playlist, songs };

    Ok(result)
}

pub fn add_playlist(conn: &Connection, playlist: DbPlaylistFull) -> Result<(), CommandError> {
    let DbPlaylistFull { playlist, songs } = playlist;

    let tx = conn.unchecked_transaction()?;

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
            ON CONFLICT(id) DO NOTHING
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
            "INSERT INTO playlist_songs (playlist_id, song_id) VALUES (?1, ?2) ON CONFLICT(playlist_id, song_id) DO NOTHING",
            params![playlist.id.to_owned(), song.id.to_owned()],
        )
        .unwrap();
    });

    tx.commit()?;

    Ok(())
}

pub fn add_local_song(conn: &Connection, path: &str, url: &str) -> Result<usize> {
    return conn.execute(
        "UPDATE songs SET path=?1, downloaded=1 WHERE url=?2",
        params![path, url],
    );
}

pub fn update_metadata(conn: &Connection, event: &WriteMetadataEvent) -> Result<()> {
    // Add metadata to song

    let path = event.src.as_str();

    for entry in event.metadata.iter() {
        let _ = match entry.key {
            MetadataKey::Title => conn.execute(
                "songs SET (title) VALUES (?1) where path = ?2",
                [entry.value.as_str().unwrap_or_default(), path],
            ),
            MetadataKey::Artist => conn.execute(
                "UPDATE songs SET (artist) VALUES (?1) where path = ?2",
                [entry.value.as_str().unwrap_or_default(), path],
            ),
            MetadataKey::Album => conn.execute(
                "UPDATE songs SET (album) VALUES (?1) where path = ?2",
                [entry.value.as_str().unwrap_or_default(), path],
            ),
            MetadataKey::AudioSourceUrl => conn.execute(
                "UPDATE songs SET (audio_source_url) VALUES (?1) where path = ?2",
                [entry.value.as_str().unwrap_or_default(), path],
            ),
            MetadataKey::FileName => {
                let new_name = entry.value.as_str().unwrap_or_default();
                let folder = std::path::Path::new(path).parent().unwrap();
                let new_path = folder.join(new_name);
                conn.execute(
                    "UPDATE songs SET (path) VALUES (?1) where path = ?2",
                    [new_path.to_str().unwrap_or_default(), path],
                )
            }
        };
    }

    Ok(())
}

pub fn set_downloaded(conn: &Connection, url: &str) -> Result<()> {
    conn.execute("UPDATE songs SET downloaded = true WHERE url = ?1", [url])?;
    Ok(())
}

pub fn update_filename(conn: &Connection, path: &str, new_path: &str) -> Result<()> {
    conn.execute(
        "UPDATE songs SET path = ?1 WHERE path = ?2",
        [new_path, path],
    )?;
    Ok(())
}
