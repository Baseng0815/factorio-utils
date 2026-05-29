use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("unknown resource type `{0}` (expected `item` or `fluid`)")]
    UnknownResourceType(String),

    #[error("invalid energy specification `{0}`")]
    InvalidEnergy(String),

    #[error("malformed prototype `{kind}/{name}`: {reason}")]
    MalformedPrototype {
        kind: &'static str,
        name: String,
        reason: String,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
