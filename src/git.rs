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

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::write_ignore_file;

    fn unique_temp_dir() -> PathBuf {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("salvo-cli-test-{}-{suffix}", std::process::id()))
    }

    #[test]
    fn write_ignore_file_creates_expected_entries() {
        let project_path = unique_temp_dir();
        fs::create_dir_all(&project_path).expect("temp project directory should be created");

        write_ignore_file(&project_path).expect(".gitignore should be written");

        let ignore_contents =
            fs::read_to_string(project_path.join(".gitignore")).expect(".gitignore should exist");
        assert_eq!(ignore_contents, "/target\n/migration/target");

        fs::remove_dir_all(&project_path).expect("temp project directory should be removed");
    }
}
