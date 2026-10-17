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
    /// The source answered with an error status, for example 503 when it is overloaded.
    Unavailable(u16),
    Format(String),
}

impl FetchError {
    pub fn message_key(&self) -> &'static str {
        match self {
            FetchError::InvalidCode => "fetch-invalid-code",
            FetchError::NotFound => "fetch-not-found",
            FetchError::Network(_) => "fetch-network",
            FetchError::Unavailable(_) => "fetch-unavailable",
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

async fn get(request: healthyteller_ingest::FetchRequest) -> Result<Vec<u8>, FetchError> {
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
    if !response.status().is_success() {
        return Err(FetchError::Unavailable(response.status().as_u16()));
    }
    response
        .bytes()
        .await
        .map(|b| b.to_vec())
        .map_err(|e| FetchError::Network(e.to_string()))
}

pub async fn product_by_barcode(code: &str) -> Result<Food, FetchError> {
    let bytes = get(OpenFoodFacts.request(code)?).await?;
    parse_product(&bytes)
}

/// Searches Open Food Facts by name or brand. Only the search terms leave the device.
/// Tries the search services in the order the adapter gives and keeps the first
/// error if all fail. Hits carry no nutrients; load the picked product with
/// [`product_by_barcode`].
pub async fn search_products(terms: &str) -> Result<Vec<Food>, FetchError> {
    let mut first_error = None;
    for request in OpenFoodFacts.search_requests(terms)? {
        match search_with(request).await {
            Ok(foods) => return Ok(foods),
            Err(e) => {
                first_error.get_or_insert(e);
            }
        }
    }
    Err(first_error.unwrap_or(FetchError::NotFound))
}

async fn search_with(request: healthyteller_ingest::FetchRequest) -> Result<Vec<Food>, FetchError> {
    let raw = RawArtifact {
        bytes: get(request).await?,
        retrieved_at: date::today(),
    };
    Ok(OpenFoodFacts.parse_search(&raw)?)
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
