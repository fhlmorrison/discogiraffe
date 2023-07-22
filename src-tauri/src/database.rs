use rusqlite::{Connection, Result};
use tauri::AppHandle;

pub struct DbSong {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub thumbnail: String,
    pub path: String,
    pub downloaded: bool,
}

pub struct DbPlaylist {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub thumbnail: String,
    pub path: String,
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
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            url TEXT NOT NULL,
            thumbnail TEXT NOT NULL,
            path TEXT NOT NULL,
            downloaded INTEGER NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS playlists (
            id INTEGER PRIMARY KEY,
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
            id INTEGER PRIMARY KEY,
            playlist_id INTEGER NOT NULL,
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
