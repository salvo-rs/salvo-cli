use anyhow::anyhow;
use anyhow::Result;
use once_cell::sync::Lazy;
use rust_i18n::t;
use std::collections::HashMap;
use std::fmt::Write;
use std::fs::{self};
use std::path::Path;
use walkdir::WalkDir;

static PATH_DESCRIPTIONS: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Cargo.toml".to_string(), t!("cargo_toml"));
    m.insert("cliff.toml".to_string(), t!("cliff_toml"));
    m.insert("deny.toml".to_string(), t!("deny_toml"));
    m.insert(".env".to_string(), t!("dot_env"));
    m.insert("config/config.toml".to_string(), t!("config_config_toml"));
    m.insert("migrations".to_string(), t!("migrations"));
    m.insert("migration".to_string(), t!("migrations"));
    m.insert("config".to_string(), t!("config"));
    m.insert("config/certs".to_string(), t!("config_certs"));
    m.insert("templates".to_string(), t!("templates"));
    m.insert("data".to_string(), t!("data"));
    m.insert("assets".to_string(), t!("assets"));
    m.insert("src".to_string(), t!("src"));
    m.insert("src/app_writer.rs".to_string(), t!("src_app_writer_rs"));
    m.insert("src/routers".to_string(), t!("src_routers"));
    m.insert("src/middleware".to_string(), t!("src_middleware"));
    m.insert("src/utils".to_string(), t!("src_utils"));
    m.insert("src/dtos".to_string(), t!("src_dtos"));
    m.insert("src/entities".to_string(), t!("src_entities"));
    m.insert("src/models".to_string(), t!("src_entities"));
    m.insert("src/services".to_string(), t!("src_services"));
    m.insert("src/config.rs".to_string(), t!("src_config_rs"));
    m.insert("src/app_error.rs".to_string(), t!("src_app_error_rs"));
    m.insert("src/main.rs".to_string(), t!("src_main_rs"));
    m.insert(
        ".github/workflows/build.yml".to_string(),
        t!("build_yml_description"),
    );
    m
});

pub fn write_directory_contents_to_markdown(output_file: &Path) -> Result<String> {
    let mut output = String::new();
    let project_name = output_file
        .parent()
        .ok_or(anyhow!("Parent directory not found."))?
        .file_name()
        .ok_or(anyhow!("Project name not found."))?
        .to_string_lossy();

    writeln!(output, "# {}", project_name)?;

    for entry in WalkDir::new(
        output_file
            .parent()
            .ok_or(anyhow!("Parent directory not found."))?,
    )
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.file_name().to_string_lossy() != "README.md")
    {
        let depth = entry.depth();
        let indent = "    ".repeat(depth.saturating_sub(1));
        let path = entry.path();
        let metadata = fs::metadata(path)?;
        if let Some(file_name) = path.file_name() {
            let file_name_str = file_name.to_string_lossy();
            let full_path = path
                .strip_prefix(
                    output_file
                        .parent()
                        .ok_or(anyhow!("Parent directory not found."))?,
                )?
                .to_string_lossy()
                .into_owned();
            let description = PATH_DESCRIPTIONS.get(&*full_path);
            let description = description
                .map(|s| format!("        ({})", s))
                .unwrap_or_default();
            if metadata.is_dir() {
                writeln!(
                    output,
                    "{}- **{}:** {} {}",
                    indent,
                    t!("dir"),
                    file_name_str,
                    description
                )?;
            } else {
                writeln!(
                    output,
                    "{}- *{}:* {} {}",
                    indent,
                    t!("file"),
                    file_name_str,
                    description
                )?;
            }
        }
    }
    Ok(output)
}
