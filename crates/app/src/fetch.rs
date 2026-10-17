//! Loading data from the original sources. Only the barcode leaves the device.

use healthyteller_core::Food;
use healthyteller_ingest::off::OpenFoodFacts;
use healthyteller_ingest::{IngestError, RawArtifact, SourceAdapter};

use crate::date;

#[derive(Clone, Debug, PartialEq)]
pub enum FetchError {
    InvalidCode,
    NotFound,
    Network(String),
    Format(String),
}

impl FetchError {
    pub fn message_key(&self) -> &'static str {
        match self {
            FetchError::InvalidCode => "fetch-invalid-code",
            FetchError::NotFound => "fetch-not-found",
            FetchError::Network(_) => "fetch-network",
            FetchError::Format(_) => "fetch-format",
        }
    }
}

impl From<IngestError> for FetchError {
    fn from(e: IngestError) -> Self {
        match e {
            IngestError::InvalidQuery(_) => FetchError::InvalidCode,
            IngestError::NotFound => FetchError::NotFound,
            other => FetchError::Format(other.to_string()),
        }
    }
}

pub async fn product_by_barcode(code: &str) -> Result<Food, FetchError> {
    let request = OpenFoodFacts.request(code)?;
    let mut builder = reqwest::Client::new().get(&request.url);
    // Browsers do not allow setting the User-Agent; they send their own.
    if !cfg!(target_arch = "wasm32") {
        for (name, value) in &request.headers {
            builder = builder.header(*name, value);
        }
    }
    let response = builder
        .send()
        .await
        .map_err(|e| FetchError::Network(e.to_string()))?;
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(FetchError::NotFound);
    }
    let bytes = response
        .bytes()
        .await
        .map_err(|e| FetchError::Network(e.to_string()))?;
    parse_product(&bytes)
}

/// Parses a product file, from the API or picked by the person.
pub fn parse_product(bytes: &[u8]) -> Result<Food, FetchError> {
    let raw = RawArtifact {
        bytes: bytes.to_vec(),
        retrieved_at: date::today(),
    };
    OpenFoodFacts
        .parse(&raw)?
        .into_iter()
        .next()
        .ok_or(FetchError::NotFound)
}
