use std::{fs::File, path::Path, io::Write};
use serde_json::json;
use anyhow::Result;
use handlebars::Handlebars;

use crate::Project;

pub fn create_project(project:Project)->Result<()>
{
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


