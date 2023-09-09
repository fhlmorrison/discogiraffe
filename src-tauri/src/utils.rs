use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("std::IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YoutubeDL error: {0}")]
    YoutubeDL(#[from] ytd_rs::error::YoutubeDLError),

    #[error("Id3 error: {0}")]
    Id3(#[from] id3::Error),

    #[error("Rusqlite error: {0}")]
    Rusqlite(#[from] rusqlite::Error),

    #[error("SerdeJson error: {0}")]
    SerdeJson(#[from] serde_json::Error),
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

use serde;

// we must manually implement serde::Serialize
impl serde::Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub fn parse_filename(input: &str) -> String {
    println!("Parsing filename: {}", input);
    return input
        .trim_end()
        .trim_end_matches("\"")
        .split("\"")
        .last()
        .unwrap()
        .to_string();
}
