// in-crate Error type
pub use crate::error::{Error, Result};

// #[allow(unused_imports)]
// pub use tracing::{debug, error, info, warn, trace};

// Wrapper struct
#[allow(dead_code)]
pub struct W<T>(pub T);

#[allow(dead_code)]
pub fn time<T>(t: &str, f: impl FnOnce() -> T) -> T {
    eprintln!("{t}: Starting");
    let start = std::time::Instant::now();
    let r = f();
    let elapsed = start.elapsed();
    eprintln!("{t}: Elapsed: {elapsed:?}");
    r
}

#[allow(dead_code)]
#[cfg(not(debug_assertions))]
fn current_path() -> Result<std::path::PathBuf> {
    std::env::current_exe()
        .map_err(|e| e.to_string())
        .map_err(Error::Generic)
}

#[allow(dead_code)]
#[cfg(debug_assertions)]
fn current_path() -> Result<std::path::PathBuf> {
    std::env::current_dir()
        .map_err(|e| e.to_string())
        .map_err(Error::Generic)
}
