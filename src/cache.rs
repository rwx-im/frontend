use std::io;
use std::path::Path;

use log::{debug, trace};
use rdedup_lib::settings::{Compression, Encryption, Hashing, Repo as RepoSettings};
use rdedup_lib::Repo;
use url::Url;

use crate::Error;

pub struct Cache {}

// This is a macro to reduce the noise of wrapping every call to [`RepoSettings`] with
// `map_err(Error::DedupError)`
macro_rules! try_set {
    ($e:expr) => {
        $e.map_err(Error::DedupError)?
    };
}

// Generate a static password for our cache repository.
//
// The password is always literal "password"
#[inline]
#[allow(clippy::unnecessary_wraps)]
fn static_password() -> io::Result<String> {
    Ok("password".to_string())
}

impl Cache {
    /// Attempts to open the given `path` as a deduplicated [`Cache`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::DedupOpenFailed`] if the given `path` could not be opened
    ///
    /// # Panics
    ///
    /// Panics if the given `path` can't be converted to a [`Url`]
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let path = path.as_ref();

        // Attempt to open the local cache as an rdedup repo
        let url = Url::from_file_path(path).unwrap();

        trace!("Opening cache repository at {}", path.to_string_lossy());
        let _repo = Repo::open(&url, None).map_err(Error::DedupOpenFailed)?;

        Ok(Cache {})
    }

    /// Initializes a new deduplicated [`Cache`] at the given `path`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::DedupRepoInitFailed`] if repo intialization fails.
    ///
    /// # Panics
    ///
    /// Panics if the given `path` can't be converted to a [`Url`]
    pub fn init<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let path = path.as_ref();

        debug!(
            "Initializing cache repository at {}",
            path.to_string_lossy()
        );

        // Attempt to open the local cache as an rdedup repo
        let mut settings = RepoSettings::new();

        try_set!(settings.set_compression(Compression::None));
        try_set!(settings.set_encryption(Encryption::None));
        try_set!(settings.set_hashing(Hashing::Blake2b));

        let url = Url::from_file_path(path).unwrap();
        let _repo = Repo::init(&url, &static_password, settings, None)
            .map_err(Error::DedupRepoInitFailed)?;

        Ok(Cache {})
    }
}
