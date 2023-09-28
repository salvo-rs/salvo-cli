use crate::Project;
use anyhow::{Context, Result};
use dialoguer::{console::Style, theme::ColorfulTheme, Select};
use handlebars::Handlebars;
use print_util::success;
use serde_json::json;
use std::{
    env,
    ffi::{OsStr, OsString},
    fs::{self, File},
    io::Write,
    path::Path,
    slice,
};

use super::{print_util, restricted_names, warning};

pub fn create_project(project: Project) -> Result<()> {
    check_name(&project.project_name)?;
    let project_name = &project.project_name;
    let project_path = Path::new(project_name);
    if project_path.exists() {
        anyhow::bail!("destination `{}` already exists", project_path.display())
    }

    check_path(project_path)?;
    let config = init_config()?;
    match config {
        Some(config) => {
            write_project_file(project_path, config, project.clone())?;

            init_git(project_path)?;

            success("🎉 Project successfully created!");
            success("🚀 You can now navigate into your project directory and start running and testing your project:");
            success(format!("1️⃣  use the command `cd {}`", project_name));
            success("✨ To run your project, use the command `cargo run`.");
            success("🎊 To test your project, use the command `cargo test`.");
        }
        None => anyhow::bail!("cli quit!"),
    }

    Ok(())
}

fn write_project_file(project_path: &Path, config: Config, project: Project) -> Result<()> {
    let handlebars = Handlebars::new();
    let is_web_site = config.template_type == TemplateType::SalvoWebSite;
    let data = json!({
        "project_name": project.project_name,
        "dependencies": {
            "anyhow": "1.0.75",
            "clia-tracing-config": "0.2.5",
            "jsonwebtoken": "8.3.0",
            "once_cell": "1.18.0",
            "salvo": {
                "version": "*",
                "features": ["anyhow", "logging", "cors", "oapi", "jwt-auth", "rustls", "catch-panic"]
            },
            "serde": "1.0.188",
            "thiserror": "1.0.48",
            "time": "0.3.28",
            "tokio": {
                "version": "1",
                "features": ["full"]
            },
            "toml": "0.8.0",
            "tracing": "0.1"
        },
        "is_web_site":is_web_site,
    });
    std::fs::create_dir_all(project_path)?;

    let src_path = project_path.join("src");
    std::fs::create_dir_all(&src_path)?;

    let main_file_path = src_path.join("main.rs");
    let main_template = include_str!("../template/src/main_template.hbs");
    let main_rendered = handlebars.render_template(main_template, &data)?;
    let mut main_file = File::create(main_file_path)?;
    main_file.write_all(main_rendered.as_bytes())?;
    let cargo_file_path = project_path.join("Cargo.toml");
    let cargo_template = include_str!("../template/src/cargo_template.hbs");
    let cargo_rendered = handlebars.render_template(cargo_template, &data)?;
    let mut cargo_file = File::create(cargo_file_path)?;
    cargo_file.write_all(cargo_rendered.as_bytes())?;
    let config_rs = include_bytes!("../template/src/config.rs");
    let mut config_file = File::create(src_path.join("config.rs"))?;
    config_file.write_all(config_rs)?;
    let app_error_rs = include_bytes!("../template/src/app_error.rs");
    let mut app_error_file = File::create(src_path.join("app_error.rs"))?;
    app_error_file.write_all(app_error_rs)?;

    let middleware_path = src_path.join("middleware");
    std::fs::create_dir_all(&middleware_path)?;
    let jwt_bytes = include_bytes!("../template/src/middleware/jwt.rs");
    let mut jwt_file = File::create(middleware_path.join("jwt.rs"))?;
    jwt_file.write_all(jwt_bytes)?;
    let mod_bytes = include_bytes!("../template/src/middleware/mod.rs");
    let mut mod_file = File::create(middleware_path.join("mod.rs"))?;
    mod_file.write_all(mod_bytes)?;

    let config_path = project_path.join("config");
    std::fs::create_dir_all(&config_path)?;
    let config_template = include_str!("../template/config/config.hbs");

    let config_toml_rendered = handlebars.render_template(config_template, &data)?;
    let mut config_file = File::create(config_path.join("config.toml"))?;
    config_file.write_all(config_toml_rendered.as_bytes())?;

    let cert_path = config_path.join("certs");
    std::fs::create_dir_all(&cert_path)?;
    let cert_template = include_str!("../template/config/certs/cert.pem");
    let mut cert_file = File::create(cert_path.join("cert.pem"))?;
    cert_file.write_all(cert_template.as_bytes())?;
    let key_path = cert_path.join("key.pem");
    let key_template = include_str!("../template/config/certs/key.pem");
    let mut key_file = File::create(key_path)?;
    key_file.write_all(key_template.as_bytes())?;
    if is_web_site {
        let template_path = project_path.join("template");
        std::fs::create_dir_all(&template_path)?;
        let hello_html_template = include_bytes!("../template/templates/hello.html");
        let mut hello_html_file = File::create(template_path.join("hello.html"))?;
        hello_html_file.write_all(hello_html_template)?;
        let handle_404_template = include_bytes!("../template/templates/404.html");
        let mut handle_404_file = File::create(template_path.join("handle_404.html"))?;
        handle_404_file.write_all(handle_404_template)?;
    }
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
    if join_paths(slice::from_ref(&OsStr::new(path)), "").is_err() {
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

pub fn init_git(project_path: &Path) -> Result<()> {
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
    let mut fp_ignore_file = File::create(fp_ignore)?;
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

#[derive(Debug)]
pub struct Config {
    pub template_type: TemplateType,
}

fn init_config() -> Result<Option<Config>> {
    let theme = ColorfulTheme {
        defaults_style: Style::new().blue(),
        prompt_style: Style::new().green().bold(),
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };
    let selections = &[
        "salvo_web_api (Default web api template) ",
        "salvo_web_site (Default web site template)",
        // "custom",
    ];
    let selection = Select::with_theme(&theme)
        .with_prompt(
            " Welcome to use salvo cli, please choose a template type\n   space bar to confirm",
        )
        .default(0)
        .items(&selections[..])
        .interact()?;
    let template_type = match selection {
        0 => TemplateType::SalvoWebApi,
        1 => TemplateType::SalvoWebSite,
        _ => anyhow::bail!("Invalid selection"),
    };
    Ok(Some(Config { template_type }))
}
#[derive(Debug, PartialEq)]
pub enum TemplateType {
    SalvoWebSite,
    SalvoWebApi,
}
