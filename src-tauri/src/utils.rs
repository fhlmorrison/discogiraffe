use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandError {
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
