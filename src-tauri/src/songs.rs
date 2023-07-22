use std::path::PathBuf;

use base64::{engine::general_purpose, Engine as _};
use id3::{Tag, TagLike};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::CommandError;

#[derive(Serialize, Deserialize)]
pub enum MetadataKey {
    Title,
    Artist,
    Album,
    AudioSourceUrl,
}

#[derive(Serialize, Deserialize)]
pub struct MetadataEntry {
    pub(crate) key: MetadataKey,
    pub(crate) value: Value,
}

#[derive(Serialize, Deserialize)]
pub struct WriteMetadataEvent {
    pub(crate) src: String,
    pub(crate) metadata: Vec<MetadataEntry>,
}

pub fn read_metadata(src: &str) -> Vec<MetadataEntry> {
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

pub fn write_metadata(event: WriteMetadataEvent) -> Result<(), CommandError> {
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
        }
    }
    tag.write_to_path(path, id3::Version::Id3v24)?;

    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct CoverArt {
    mime_type: String,
    b64: String,
}

pub fn read_cover_art(src: &str) -> Result<CoverArt, CommandError> {
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
