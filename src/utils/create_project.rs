use anyhow::{Result, Context};
use handlebars::Handlebars;
use serde_json::json;
use std::{fs::{File, self}, io::Write, path::Path, ffi::{OsStr, OsString}, env, slice};

use crate::Project;

use super::{restricted_names, warning, print_util};

pub fn create_project(project: Project) -> Result<()> {

    check_name(&project.project_name)?;
    let project_name = project.project_name;
    let project_path = Path::new(&project_name);
    if project_path.exists() {
        anyhow::bail!(
            "destination `{}` already exists",
             project_path.display()
        )
    }

    check_path(&project_path)?;
    init_git(&project_path)?;

    let handlebars = Handlebars::new();

    let data = json!({
        "dependencies": {
            "salvo": "0.55",
            "tokio": { "version": "1", "features": ["full"] },
            "tracing": "0.1",
            "tracing-subscriber": "0.3"
        }
    });


    std::fs::create_dir_all(&project_path)?;

    let src_path = project_path.join("src");
    std::fs::create_dir_all(&src_path)?;

    let main_file_path = src_path.join("main.rs");
    let main_template = include_str!("../template/main_template.hbs");
    let main_rendered = handlebars.render_template(main_template, &data)?;
    let mut main_file = File::create(main_file_path)?;
    main_file.write_all(main_rendered.as_bytes())?;

    let cargo_file_path = project_path.join("Cargo.toml");
    let cargo_template = include_str!("../template/cargo_template.hbs");
    let cargo_rendered = handlebars.render_template(cargo_template, &data)?;
    let mut cargo_file = File::create(cargo_file_path)?;
    cargo_file.write_all(cargo_rendered.as_bytes())?;

    Ok(())
}

fn check_name(name: &str) -> Result<()> {

    restricted_names::validate_package_name(name, "package name")?;

    if restricted_names::is_keyword(name) {
        anyhow::bail!(
            "the name `{}` cannot be used as a package name, it is a Rust keyword",
            name,
        );
    }
    if restricted_names::is_conflicting_artifact_name(name) {
        warning(format!(
            "the name `{}` will not support binary \
            executables with that name, \
            it conflicts with cargo's build directory names",
            name
        ));
    }
    if name == "test" {
        anyhow::bail!(
            "the name `test` cannot be used as a package name, \
            it conflicts with Rust's built-in test library",
        );
    }
    if ["core", "std", "alloc", "proc_macro", "proc-macro"].contains(&name) {
        warning(format!(
            "the name `{}` is part of Rust's standard library\n\
            It is recommended to use a different name to avoid problems.",
            name,
        ));
    }
    if restricted_names::is_windows_reserved(name) {
        if cfg!(windows) {
            anyhow::bail!(
                "cannot use name `{}`, it is a reserved Windows filename",
                name,
            );
        } else {
            warning(format!(
                "the name `{}` is a reserved Windows filename\n\
                This package will not work on Windows platforms.",
                name
            ));
        }
    }
    if restricted_names::is_non_ascii_name(name) {
        warning(format!(
            "the name `{}` contains non-ASCII characters\n\
            Non-ASCII crate names are not supported by Rust.",
            name
        ));
    }
    Ok(())
}
fn check_path(path: &Path) -> Result<()> {
    // warn if the path contains characters that will break `env::join_paths`
    if let Err(_) = join_paths(slice::from_ref(&OsStr::new(path)), "") {
        let path = path.to_string_lossy();
        print_util::warning(format!(
            "the path `{path}` contains invalid PATH characters (usually `:`, `;`, or `\"`)\n\
            It is recommended to use a different name to avoid problems."
        ));
    }
    Ok(())
}

pub fn join_paths<T: AsRef<OsStr>>(paths: &[T], env: &str) -> Result<OsString> {
    env::join_paths(paths.iter()).with_context(|| {
        let mut message = format!(
            "failed to join paths from `${env}` together\n\n\
             Check if any of path segments listed below contain an \
             unterminated quote character or path separator:"
        );
        for path in paths {
            use std::fmt::Write;
            write!(&mut message, "\n    {:?}", Path::new(path)).unwrap();
        }
        message
    })
}

pub fn init_git(project_path:&Path)->Result<()> {
    if !project_path.join(".git").exists() {
        // Temporary fix to work around bug in libgit2 when creating a
        // directory in the root of a posix filesystem.
        // See: https://github.com/libgit2/libgit2/issues/5130
        create_dir_all(project_path)?;
        git2::Repository::init(project_path)?;
        write_ignore_file(project_path)?;
    }
    Ok(())

}


fn write_ignore_file(project_path: &Path) -> Result<()> {
    let fp_ignore = project_path.join(".gitignore");
    let mut fp_ignore_file = File::create(&fp_ignore)?;
    fp_ignore_file.write_all(b"/target\n")?;
    Ok(())
}


/// Equivalent to [`std::fs::create_dir_all`] with better error messages.
pub fn create_dir_all(p: impl AsRef<Path>) -> Result<()> {
    _create_dir_all(p.as_ref())
}

fn _create_dir_all(p: &Path) -> Result<()> {
    fs::create_dir_all(p)
        .with_context(|| format!("failed to create directory `{}`", p.display()))?;
    Ok(())
}