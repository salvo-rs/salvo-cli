use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use crate::printer::{gray, warning};
use crate::{git, Project};
use anyhow::Result;
use liquid::model::Object;
use rust_i18n::t;

pub(crate) mod selection;
use selection::Selected;

#[derive(rust_embed::RustEmbed)]
#[folder = "./templates/classic"]
struct Template;

pub fn generate(proj: &Project) -> Result<()> {
    let Some(config) = selection::get_selected()? else {
        anyhow::bail!(t!("error_cli_quit"))
    };
    let project_path = Path::new(&proj.name);
    match git::init_repository(project_path) {
        Ok(_) => {}
        Err(e) => {
            warning(t!("warning_init_git", error = e).replace(r"\n", "\n"));
        }
    }

    create_files(project_path, config, proj)?;
    Ok(())
}

pub(crate) fn create_files(project_path: &Path, selected: Selected, proj: &Project) -> Result<()> {
    let db_lib = selected.db_lib.to_string();
    let db_type = selected.db_type.to_string();
    let data = liquid::object!({
        "project_name": proj.name,
        "db_type":db_type,
        "db_lib":db_lib,
        "main_log_message":t!("main_log_message"),
        "config_error_no_exits":t!("config_error_no_exits"),
        "config_error_read":t!("config_error_read"),
        "config_error_parse":t!("config_error_parse"),
        "config_error_read_failed":t!("config_error_read_failed"),
        "username":t!("username"),
        "password":t!("password"),
        "incorrect_password":t!("incorrect_password"),
        "login":t!("login"),
        "user":t!("user"),
        "add_user":t!("add_user"),
        "lang":t!("lang"),
        "rbatis_website":t!("rbatis_website"),
        "account":t!("account"),
        "password":t!("password"),
        "you_wont_be_able_to_revert_this":t!("you_wont_be_able_to_revert_this"),
        "search_placeholder":t!("search_placeholder"),
        "search":t!("search"),
        "previous_page":t!("previous_page"),
        "page":t!("page"),
        "total_records":t!("total_records"),
        "page_size":t!("page_size"),
        "update":t!("update"),
        "delete":t!("delete"),
        "next_page":t!("next_page"),
        "talk_to_me_lang":t!("talk_to_me_lang"),
        "salvo_cli_welcome":t!("salvo_cli_welcome"),
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
        "mongodb_website": t!("mongodb_website"),
        "mongodb_usage_import_user_data": t!("mongodb_usage_import_user_data"),
        "initialization": t!("initialization"),
        "initialization_sqlx_cli_not_sqlite": t!("initialization_sqlx_cli_not_sqlite").replace(r"\n", "\n"),
        "initialization_seaorm_cli_not_sqlite": t!("initialization_seaorm_cli_not_sqlite").replace(r"\n", "\n"),
        "initialization_diesel_cli_not_sqlite": t!("initialization_diesel_cli_not_sqlite").replace(r"\n", "\n"),
        "initialization_rbatis_cli_not_sqlite": t!("initialization_rbatis_cli_not_sqlite").replace(r"\n", "\n"),
        "seaorm_cli_website": t!("seaorm_cli_website").replace(r"\n", "\n"),
        "diesel_cli_website": t!("diesel_cli_website").replace(r"\n", "\n"),
    });

    let db_lib_str = &*selected.db_lib.to_string();
    for filename in Template::iter() {
        if filename.starts_with("_base/") {
            let file = Template::get(filename.as_ref()).expect("file must exist");
            let file_path = project_path.join(filename.as_ref().trim_start_matches("_base/"));
            write_file(&file.data, &file_path, &data)?;
        } else if filename.starts_with("_data/") {
            if filename.contains(db_lib_str) && filename.contains(".sqlite") && db_type == "sqlite"
            {
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
        let msg = t!("rendering_liquid_file").replace(r"\n", "\n") + &format!(" {:?}", file_path);
        gray(msg);
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
