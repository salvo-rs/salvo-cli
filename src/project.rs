use std::ffi::{OsStr, OsString};
use std::path::Path;
use std::{env, slice};

use anyhow::{Context, Result};
use rust_i18n::t;

use crate::printer::{self, success, warning};
use crate::{Project, namer};

pub fn create(proj: &Project) -> Result<()> {
    check_name(&proj.name)?;
    let project_path = Path::new(&proj.name);
    if project_path.exists() {
        anyhow::bail!(t!(
            "error_project_path_exist",
            path = project_path.to_string_lossy()
        ))
    }

    check_path(project_path)?;
    crate::templates::classic::generate(proj)?;
    after_print_info(&proj.name);
    Ok(())
}

fn after_print_info(project_name: &String) {
    println!(); // a new line
    success(t!("create_info", project_name = project_name).replace(r"\n", "\n"));
    success(t!("create_success").replace(r"\n", "\n"));
    success(t!("rust_version_tip"));
    success(t!("cursor_rules_info").replace(r"\n", "\n"));
    println!(); // a new line
}

fn check_name(name: &str) -> Result<()> {
    namer::validate_package_name(name, "package name")?;

    if namer::is_keyword(name) {
        anyhow::bail!(t!("error_is_keyword", name = name));
    }
    if namer::is_conflicting_artifact_name(name) {
        warning(t!("error_is_conflicting_artifact_name", name = name).replace(r"\n", "\n"));
    }
    if name == "test" {
        anyhow::bail!(t!("error_equal_test").replace(r"\n", "\n"))
    }
    if ["core", "std", "alloc", "proc_macro", "proc-macro"].contains(&name) {
        warning(t!("error_part_of_standard_library", name = name,).replace(r"\n", "\n"));
    }
    if namer::is_windows_reserved(name) {
        if cfg!(windows) {
            anyhow::bail!(t!("error_is_windows_reserved", name = name),);
        } else {
            warning(t!("warning_is_windows_reserved", name = name).replace(r"\n", "\n"));
        }
    }
    if namer::is_non_ascii_name(name) {
        warning(t!("warning_is_non_ascii_name", name = name).replace(r"\n", "\n"));
    }
    Ok(())
}
fn check_path(path: &Path) -> Result<()> {
    // warn if the path contains characters that will break `env::join_paths`
    if join_paths(slice::from_ref(&OsStr::new(path)), "").is_err() {
        let path = path.to_string_lossy();
        printer::warning(t!("warning_invalid_path", path = path));
    }
    Ok(())
}

fn join_paths<T: AsRef<OsStr>>(paths: &[T], env: &str) -> Result<OsString> {
    env::join_paths(paths.iter()).with_context(|| {
        let mut message = t!("erroe_join_paths", env = env).replace(r"\n", "\n");
        for path in paths {
            use std::fmt::Write;
            write!(&mut message, "\n    {:?}", Path::new(path)).unwrap();
        }
        message
    })
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{check_name, check_path, join_paths};

    #[test]
    fn check_name_rejects_reserved_names() {
        assert!(check_name("test").is_err());
        assert!(check_name("async").is_err());
    }

    #[test]
    fn check_name_handles_windows_reserved_names_per_platform() {
        let result = check_name("con");
        if cfg!(windows) {
            assert!(result.is_err());
        } else {
            assert!(result.is_ok());
        }
    }

    #[test]
    fn check_path_accepts_normal_project_paths() {
        assert!(check_path(Path::new("salvo-demo")).is_ok());
    }

    #[test]
    fn join_paths_includes_bad_path_in_error_message() {
        let invalid_segment = if cfg!(windows) {
            "bad\"path"
        } else {
            "bad:path"
        };
        let err = join_paths(&[invalid_segment], "PATH").unwrap_err();
        let message = err.to_string();
        assert!(message.contains("PATH"));
        assert!(message.contains("bad"));
    }
}
