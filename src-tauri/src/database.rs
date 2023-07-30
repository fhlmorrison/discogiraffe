use rusqlite::{Connection, Result};
use tauri::AppHandle;

use crate::songs::{MetadataKey, WriteMetadataEvent};

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
}

pub struct DbPlaylist {
    pub id: String,
    pub title: String,
    pub description: String,
    pub url: String,
    pub thumbnail: Option<String>,
    pub path: Option<String>,
    pub downloaded: bool,
}

pub struct DbPlaylistSong {
    pub id: i32,
    pub playlist_id: i32,
    pub song_id: i32,
}

pub struct AppState {
    pub db: std::sync::Mutex<Option<Connection>>,
}

pub fn init_db(handle: &AppHandle) -> Result<Connection> {
    let dir = handle
        .path_resolver()
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
            path TEXT NOT NULL,
            downloaded INTEGER NOT NULL,
            artist TEXT,
            album TEXT,
            audio_source_url TEXT
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS playlists (
            id STRING PRIMARY KEY,
            title TEXT NOT NULL,
            url TEXT NOT NULL,
            thumbnail TEXT NOT NULL,
            path TEXT NOT NULL,
            downloaded INTEGER NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS playlist_songs (
            id STRING PRIMARY KEY,
            playlist_id STRING NOT NULL,
            song_id INTEGER NOT NULL,
            FOREIGN KEY (playlist_id) REFERENCES playlists(id),
            FOREIGN KEY (song_id) REFERENCES songs(id)
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
        })
    });
}

pub fn get_playlists(conn: &Connection) -> Result<Vec<DbPlaylist>> {
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

pub fn get_playlist_songs(conn: &Connection) -> Result<Vec<DbPlaylistSong>> {
    let mut stmt = conn.prepare("SELECT * FROM playlist_songs WHERE playlist_id=?1")?;

    let rows = stmt.query_map([], |row| {
        Ok(DbPlaylistSong {
            id: row.get(0)?,
            playlist_id: row.get(1)?,
            song_id: row.get(2)?,
        })
    })?;

    let result = rows.into_iter().collect::<Result<Vec<_>, _>>()?;

    Ok(result)
}

pub fn update_metadata(conn: &Connection, event: &WriteMetadataEvent) -> Result<()> {
    // Add metadata to song

    let path = event.src.as_str();

    for entry in event.metadata.iter() {
        let _ = match entry.key {
            MetadataKey::Title => conn.execute(
                "INSERT INTO songs (path, title) VALUES (?2, ?1) ON CONFLICT(path) UPDATE songs SET (title) VALUES (?1)",
                [entry.value.as_str().unwrap_or_default(), path],
            ),
            MetadataKey::Artist => conn.execute(
                "INSERT INTO songs (path, artist) VALUES (?2, ?1) ON CONFLICT(path) DO UPDATE songs SET (artist) VALUES (?1)",
                [entry.value.as_str().unwrap_or_default(), path],
            ),
            MetadataKey::Album => conn.execute(
                "INSERT INTO songs (path, album) VALUES (?2, ?1) ON CONFLICT(path) DO UPDATE songs SET (album) VALUES (?1)",
                [entry.value.as_str().unwrap_or_default(), path],
            ),
            MetadataKey::AudioSourceUrl => conn.execute(
                "INSERT INTO songs (path, audio_source_url) VALUES (?2, ?1) ON CONFLICT(path) DO UPDATE songs SET (audio_source_url) VALUES (?1)",
                [entry.value.as_str().unwrap_or_default(), path],
            ),
        };
    }

    Ok(())
}
