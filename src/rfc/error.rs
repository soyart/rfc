use thiserror::Error;

#[derive(Debug, Error)]
pub enum RfcError {
    #[error("io error")]
    IoError(std::io::Error),

    #[error("serialize error")]
    Serialize(String),

    #[error("deserialize error")]
    Deserialize(String),

    #[error("encryption error")]
    Encryption,

    #[error("decryption error")]
    Decryption,

    #[error("compression error")]
    Compression,

    #[error("decompression error")]
    Decompression,
}