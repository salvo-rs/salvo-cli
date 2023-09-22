use anyhow::Result;
use handlebars::Handlebars;
use serde_json::json;
use std::{fs::File, io::Write, path::Path};

use crate::Project;

use super::{restricted_names, warning};

pub fn create_project(project: Project) -> Result<()> {

    check_name(&project.project_name)?;
    let handlebars = Handlebars::new();

    let data = json!({
        "dependencies": {
            "salvo": "0.55",
            "tokio": { "version": "1", "features": ["macros"] },
            "tracing": "0.1",
            "tracing_subscriber": "0.3"
        }
    });
    let project_name = project.project_name;
    let project_path = Path::new(&project_name);
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
