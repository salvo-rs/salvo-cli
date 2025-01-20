use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Result;

use crate::utils;

pub fn init_repository(dir: &Path) -> Result<()> {
    if !dir.join(".git").exists() {
        // Temporary fix to work around bug in libgit2 when creating a
        // directory in the root of a posix filesystem.
        // See: https://github.com/libgit2/libgit2/issues/5130
        utils::create_dir_all(dir)?;
        git2::Repository::init(dir)?;
        write_ignore_file(dir)?;
    }
    Ok(())
}

pub fn write_ignore_file(project_path: &Path) -> Result<()> {
    let fp_ignore = project_path.join(".gitignore");
    let mut fp_ignore_file = File::create(fp_ignore)?;
    fp_ignore_file.write_all(b"/target\n/migration/target")?;
    Ok(())
}