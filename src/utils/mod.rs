use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

/// Equivalent to [`create_dir_all`] with better error messages.
pub fn create_dir_all(p: impl AsRef<Path>) -> Result<()> {
    let p = p.as_ref();
    fs::create_dir_all(p)
        .with_context(|| format!("failed to create directory `{}`", p.display()))?;
    Ok(())
}
