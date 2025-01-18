use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use liquid::model::Object;
use rust_i18n::t;

use crate::printer::warning;
use crate::{git, NewCmd};

mod selection;
use selection::Selected;

#[derive(rust_embed::RustEmbed)]
#[folder = "./templates/classic"]
struct Template;

pub fn generate(new_cmd: &NewCmd) -> Result<()> {
    let Some(config) = selection::get_selected()? else {
        anyhow::bail!("cli quit!")
    };
    let project_path = Path::new(&new_cmd.project_name);
    match git::init_repository(project_path) {
        Ok(_) => {}
        Err(e) => {
            warning(t!("warning_init_git", error = e).replace(r"\n", "\n"));
        }
    }

    create_files(project_path, config, new_cmd)?;
    Ok(())
}

fn create_files(project_path: &Path, user_selected: Selected, new_cmd: &NewCmd) -> Result<()> {
    let db_lib = user_selected.db_lib.to_string();
    let db_type = user_selected.db_type.to_string();
    let data = liquid::object!({
        "project_name": new_cmd.project_name,
        "db_type":db_type,
        "db_lib":db_lib,
        "main_log_message":t!("main_log_message"),
        "config_error_no_exits":t!("config_error_no_exits"),
        "config_error_read":t!("config_error_read"),
        "config_error_parse":t!("config_error_parse"),
        "config_error_read_failed":t!("config_error_read_failed"),
        "generate_a_string_of_a_specified_length":t!("generate_a_string_of_a_specified_length"),
        "username":t!("username"),
        "password":t!("password"),
        "incorrect_password":t!("incorrect_password"),
        "login":t!("login"),
        "user":t!("user"),
        "add_user":t!("add_user"),
        "user_list":t!("user_list"),
        "are_you_sure_you_want_to_delete":t!("are_you_sure_you_want_to_delete"),
        "page_not_found":t!("page_not_found"),
        "contact_support":t!("contact_support"),
        "return_to_homepage":t!("return_to_homepage"),
        "delete":t!("delete"),
        "yes":t!("yes"),
        "cancel":t!("cancel"),
        "open_api_page":t!("open_api_page"),
        "login_page":t!("login_page"),
        "operation":t!("operation"),
        "create_success_seaorm__mysql_or_pgsql_install_seaorm":t!("create_success_seaorm__mysql_or_pgsql_install_seaorm"),
        "create_success_mysql_or_pgsql_fist_use":t!("create_success_mysql_or_pgsql_fist_use").replace(r"\n", "\n"),
        "create_success_seaorm__mysql_or_pgsql_fist_use":t!("create_success_seaorm__mysql_or_pgsql_fist_use").replace(r"\n", "\n"),
        "create_success_diesel__mysql_or_pgsql_fist_use":t!("create_success_diesel__mysql_or_pgsql_fist_use").replace(r"\n", "\n"),

        "is_starting": t!("is_starting"),
        "listen_on": t!("listen_on"),
        "database_connection_failed": t!("database_connection_failed"),
        "user_does_not_exist": t!("user_does_not_exist"),
        "rust_version_tip": t!("rust_version_tip"),
        "introduction_title": t!("introduction_title"),
        "introduction_content": t!("introduction_content"),
        "seleted_sqlite": t!("seleted_sqlite"),
        "run_the_project": t!("run_the_project"),
        "run_the_tests": t!("run_the_tests"),
        "sqlx_cli": t!("sqlx_cli"),
        "about_salvo": t!("about_salvo"),
        "about_salvo_text": t!("about_salvo_text"),
        "tip_title": t!("tip_title"),
        "password_tip": t!("password_tip"),
        "config_tip": t!("config_tip"),
        "orm_title": t!("orm_title"),
        "sqlx_website": t!("sqlx_website"),
        "seaorm_website": t!("seaorm_website"),
        "diesel_website": t!("diesel_website"),
        "rbatis_website": t!("rbatis_website"),
        "mongodb_website": t!("mongodb_website"),
        "initialization": t!("initialization"),
        "initialization_sqlx_cli_not_sqlite": t!("initialization_sqlx_cli_not_sqlite").replace(r"\n", "\n"),
        "initialization_seaorm_cli_not_sqlite": t!("initialization_seaorm_cli_not_sqlite").replace(r"\n", "\n"),
        "initialization_diesel_cli_not_sqlite": t!("initialization_diesel_cli_not_sqlite").replace(r"\n", "\n"),
        "initialization_rbatis_cli_not_sqlite": t!("initialization_rbatis_cli_not_sqlite").replace(r"\n", "\n"),
        "seaorm_cli_website": t!("seaorm_cli_website").replace(r"\n", "\n"),
        "diesel_cli_website": t!("diesel_cli_website").replace(r"\n", "\n"),
        "mongodb_usage_import_user_data": t!("mongodb_usage_import_user_data").replace(r"\n", "\n")
    });

    let db_lib_str = &*user_selected.db_lib.to_string();
    for filename in Template::iter() {
        if filename.starts_with("_base/") {
            let file = Template::get(filename.as_ref()).expect("file must exist");
            let file_path = project_path.join(filename.as_ref().trim_start_matches("_base/"));
            write_file(&file.data, &file_path, &data)?;
        } else if filename.starts_with("_data/") {
            if filename.contains(db_lib_str) && filename.contains(db_lib_str) {
                let file = Template::get(filename.as_ref()).expect("file must exist");
                let file_path = project_path.join(filename.as_ref().trim_start_matches("_"));
                write_file(&file.data, &file_path, &data)?;
            }
        } else if filename.starts_with(&format!("{}/", db_lib_str)) {
            let file = Template::get(filename.as_ref()).expect("file must exist");
            let file_path = project_path.join(
                filename
                    .as_ref()
                    .trim_start_matches(&format!("{}/", db_lib_str)),
            );
            write_file(&file.data, &file_path, &data)?;
        }
    }

    Ok(())
}

fn write_file(tmpl: &[u8], file_path: &Path, data: &Object) -> Result<()> {
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }
    if file_path.extension() == Some(OsStr::new("liquid")) {
        println!("rendering liquid file: {:?}", file_path);
        let template = liquid::ParserBuilder::with_stdlib()
            .build()
            .expect("should create liquid parser")
            .parse(&String::from_utf8_lossy(tmpl))?;
        let rendered = template.render(data)?;
        let mut target_file =
            File::create(file_path.to_string_lossy().trim_end_matches(".liquid"))?;
        target_file.write_all(rendered.as_bytes())?;
    } else {
        let mut target_file = File::create(file_path)?;
        target_file.write_all(tmpl)?;
    }
    Ok(())
}
