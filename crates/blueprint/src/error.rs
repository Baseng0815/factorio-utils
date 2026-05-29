use thiserror::Error;

use crate::world::EntityName;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("base64 decode error: {0}")]
    Base64(#[from] base64::DecodeError),

    #[error("zlib error: {0}")]
    Zlib(String),

    #[error("blueprint string is empty (missing version byte)")]
    MissingVersionByte,

    #[error("unsupported blueprint string version `{0}`")]
    UnsupportedVersion(char),

    #[error("unsupported envelope `{0}` (only `blueprint` is supported)")]
    UnsupportedEnvelope(&'static str),

    #[error("unsupported entity `{0}` (no rich type for this prototype family)")]
    UnsupportedEntity(EntityName),

    #[error("malformed entity #{number}: {reason}")]
    MalformedEntity { number: u64, reason: String },

    #[error("duplicate entity_number {0}")]
    DuplicateEntityNumber(u64),
}

pub type Result<T> = std::result::Result<T, Error>;
