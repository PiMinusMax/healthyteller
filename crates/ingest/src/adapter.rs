//! The contract every source adapter implements.

use std::fmt;

use healthyteller_core::Food;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceMeta {
    pub id: &'static str,
    pub name: &'static str,
    pub url: &'static str,
    pub license: &'static str,
    pub citation: &'static str,
}

/// Raw bytes as they came from the source or from a file the user picked.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawArtifact {
    pub bytes: Vec<u8>,
    /// ISO 8601 date of retrieval.
    pub retrieved_at: String,
}

/// What the app has to request to load data from the original source.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FetchRequest {
    pub url: String,
    pub headers: Vec<(&'static str, String)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IngestError {
    /// The source has no direct download; the user imports a file instead.
    NotSupported,
    NotFound,
    InvalidQuery(String),
    Format(String),
}

impl fmt::Display for IngestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IngestError::NotSupported => write!(f, "source has no direct download"),
            IngestError::NotFound => write!(f, "not found at the source"),
            IngestError::InvalidQuery(q) => write!(f, "invalid query: {q}"),
            IngestError::Format(e) => write!(f, "unexpected format: {e}"),
        }
    }
}

impl std::error::Error for IngestError {}

pub trait SourceAdapter {
    fn meta(&self) -> SourceMeta;
    /// Request for loading `query` from the original source. May return `NotSupported`.
    fn request(&self, query: &str) -> Result<FetchRequest, IngestError>;
    /// File import is mandatory as the fallback path.
    fn import_file(&self, bytes: &[u8], retrieved_at: &str) -> Result<RawArtifact, IngestError> {
        Ok(RawArtifact {
            bytes: bytes.to_vec(),
            retrieved_at: retrieved_at.to_owned(),
        })
    }
    fn parse(&self, raw: &RawArtifact) -> Result<Vec<Food>, IngestError>;
}
