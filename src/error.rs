use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Deduplication error")]
    DedupError(#[source] io::Error),
    #[error("Could not initialize the deduplication store: {0}")]
    DedupRepoInitFailed(#[source] io::Error),
    #[error("Could not open path as dedup repository: {0}")]
    DedupOpenFailed(#[source] io::Error),
}
