use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Result, Write};
use std::path::Path;
use walkdir::WalkDir;

// 使用 once_cell 创建静态的 HashMap，存储全路径和描述
static PATH_DESCRIPTIONS: Lazy<HashMap<String, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("/Cargo.toml".to_string(), "这是 Cargo.toml 的描述");
    m.insert(
        "/migrations".to_string(),
        "这是 migrations 目录的描述",
    );
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
            let description = PATH_DESCRIPTIONS.get(full_path).unwrap_or(&"");
            if metadata.is_dir() {
                writeln!(
                    file,
                    "{}- **Dir:** {} {}",
                    indent, file_name_str, description
                )?;
            } else {
                writeln!(
                    file,
                    "{}- *File:* {} {}",
                    indent, file_name_str, description
                )?;
            }
        }
    }
    Ok(())
}
