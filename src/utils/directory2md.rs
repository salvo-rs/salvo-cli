use once_cell::sync::Lazy;
use rust_i18n::t;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Result, Write};
use std::path::Path;
use walkdir::WalkDir;

// 使用 once_cell 创建静态的 HashMap，存储全路径和描述
static PATH_DESCRIPTIONS: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("/Cargo.toml".to_string(), t!("cargo_toml"));
    m.insert("/.env".to_string(), t!("dot_env"));
    m.insert("/config/config.toml".to_string(), t!("config_config_toml"));

    m.insert("/migrations".to_string(), t!("migrations"));
    m.insert("/config".to_string(), t!("config"));
    m.insert("/config/certs".to_string(), t!("config_certs"));
    m.insert("/templates".to_string(), t!("templates"));
    m.insert("/data".to_string(), t!("data"));
    m.insert("/assets".to_string(), t!("assets"));
    m.insert("/src".to_string(), t!("src"));
    m.insert(
        "/src/app_response.rs".to_string(),
        t!("src_app_response_rs"),
    );
    m.insert("/src/routers".to_string(), t!("src_routers"));
    m.insert("/src/middleware".to_string(), t!("src_middleware"));
    m.insert("/src/utils".to_string(), t!("src_utils"));
    m.insert("/src/dtos".to_string(), t!("src_dtos"));
    m.insert("/src/entities".to_string(), t!("src_entities"));
    m.insert("/src/services".to_string(), t!("src_services"));
    m.insert("/src/config.rs".to_string(), t!("src_config_rs"));
    m.insert("/src/app_error.rs".to_string(), t!("src_app_error_rs"));
    m.insert("/src/main.rs".to_string(), t!("src_main_rs"));
    m
});

pub fn write_directory_contents_to_markdown(output_file: &Path) -> Result<()> {
    let mut file = File::create(output_file)?;
    let project_name = output_file
        .parent()
        .unwrap()
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();
    writeln!(file, "# {}", project_name)?;

    for entry in WalkDir::new(output_file.parent().unwrap())
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
            let full_path = path.to_string_lossy().into_owned();
            let full_path = full_path.trim_start_matches(&*project_name);
            dbg!(&full_path);
            let description = PATH_DESCRIPTIONS.get(full_path);
            let description = description
                .map(|s| format!("        ({})", s))
                .unwrap_or_default();
            if metadata.is_dir() {
                writeln!(
                    file,
                    "{}- **{}:** {} {}",
                    indent,
                    t!("dir"),
                    file_name_str,
                    description
                )?;
            } else {
                writeln!(
                    file,
                    "{}- *{}:* {} {}",
                    indent,
                    t!("file"),
                    file_name_str,
                    description
                )?;
            }
        }
    }
    Ok(())
}
