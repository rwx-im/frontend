#![allow(clippy::pub_enum_variant_names)]

use std::path::{Path, PathBuf};
use std::{env, io};

use anyhow::Context;
mod cache;
mod error;
mod http;

pub use cache::Cache;
pub use error::Error;

fn abs_path<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    let path = path.as_ref();

    let abs_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    };

    Ok(abs_path)
}

/// Attempts to open the given `path` as a cache repository.
///
/// If the given `path` does not exist, a new repository will be initialized there, if possible.
fn prepare_cache<P: AsRef<Path>>(path: P) -> Result<Cache, Error> {
    let cache = Cache::open(path.as_ref());

    match cache {
        Ok(cache) => Ok(cache),
        Err(Error::DedupOpenFailed(inner)) => {
            if inner.kind() == io::ErrorKind::NotFound {
                Cache::init(path)
            } else {
                Err(Error::DedupError(inner))
            }
        }
        Err(e) => Err(e),
    }
}

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    // Override RUST_LOG with a default setting if it's not set by the user
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "actix_web=debug,rwx_im=trace");
    }

    env_logger::init();

    // Open the local cache
    let path = abs_path("cache").unwrap();
    let _cache = prepare_cache(path).with_context(|| "Could not prepare cache")?;

    // Start the HTTP server
    http::start_http_server().await?;

    Ok(())
}
