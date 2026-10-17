//! One adapter per data source, plus normalisation and matching between sources.
//!
//! Every adapter supports loading from the original source and importing a file
//! the user picked (AGENTS.md rule 12). Network access itself happens in the app,
//! so this crate stays testable without a network.

pub mod adapter;
pub mod off;
pub mod rules;
pub mod sources;

pub use adapter::{FetchRequest, IngestError, RawArtifact, SourceAdapter, SourceMeta};
