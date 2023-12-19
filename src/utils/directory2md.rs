use std::fs::{self, File};
use std::io::{Result, Write};
use std::path::Path;
use walkdir::WalkDir;

pub fn write_directory_contents_to_markdown(output_file: &Path) -> Result<()> {
    let mut file = File::create(output_file)?;
    writeln!(file, "# Directory Contents\n")?;
    for entry in WalkDir::new(output_file.parent().unwrap())
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path() != Path::new("./READEME.md"))
    // Exclude the output file itself
    {
        let depth = entry.depth();
        let indent = "    ".repeat(depth.saturating_sub(1)); // Markdown uses 4 spaces for each list level
        let path = entry.path();
        let metadata = fs::metadata(path)?;
        dbg!(&path);
        if let Some(file_name) = path.file_name() {
            let file_name = file_name.to_string_lossy();
            if metadata.is_dir() {
                // Use `**Dir:**` to denote directories
                writeln!(file, "{}- **Dir:** {}", indent, file_name)?;
            } else {
                // Use `*File:*` to denote files
                writeln!(file, "{}- *File:* {}", indent, file_name)?;
            }
        }
    }
    Ok(())
}
