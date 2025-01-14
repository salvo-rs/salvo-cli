use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::{env, slice};

use anyhow::{Context, Result};
use print_util::success;
use rust_i18n::t;
use serde_json::{json, Value as JsonValue};

use super::get_selection::{get_user_selected, CodeGen, DbLib, DbType, UserSelected};
use super::{print_util, restricted_names, warning};
use crate::Project;

#[derive(rust_embed::RustEmbed)]
#[folder = "./template"]
struct Template;

pub fn create_project(project: Project) -> Result<()> {
    check_name(&project.project_name)?;
    let project_name = &project.project_name;
    let project_path = Path::new(project_name);
    if project_path.exists() {
        anyhow::bail!(t!(
            "error_project_path_exist",
            path = project_path.to_string_lossy()
        ))
    }

    check_path(project_path)?;
    let config = get_user_selected()?;
    match config {
        Some(config) => {
            write_project_file(project_path, config, project.clone())?;

            match init_git(project_path) {
                Ok(_) => {}
                Err(e) => {
                    warning(t!("warning_init_git", error = e).replace(r"\n", "\n"));
                }
            }
            after_print_info(project_name);
        }
        None => anyhow::bail!("cli quit!"),
    }
    Ok(())
}

fn after_print_info(project_name: &String) {
    println!(); // a new line
    success(t!("create_info", project_name = project_name).replace(r"\n", "\n"));
    success(t!("create_success").replace(r"\n", "\n"));
    success(t!("rust_version_tip"));
    println!(); // a new line
}

pub fn write_project_file(
    project_path: &Path,
    user_selected: UserSelected,
    project: Project,
) -> Result<()> {
    let code_gen = user_selected.code_gen.to_string();
    let db_lib = user_selected.db_lib.to_string();
    let db_type = user_selected.db_type.to_string();
    let mut data = json!({
        "project_name": project.project_name,
        "code_gen":code_gen,
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
    });
    data["is_starting"] = json_string(t!("is_starting"));
    data["listen_on"] = json_string(t!("listen_on"));
    data["database_connection_failed"] = json_string(t!("database_connection_failed"));
    data["user_does_not_exist"] = json_string(t!("user_does_not_exist"));
    data["rust_version_tip"] = json_string(t!("rust_version_tip"));
    data["project_dir_description"] = json_string(t!("project_dir_description"));
    data["introduction"] = json_string(t!("introduction"));
    data["introduction_text"] = json_string(t!("introduction_text"));
    data["seleted_sqlite"] = json_string(t!("seleted_sqlite"));
    data["run_the_project"] = json_string(t!("run_the_project"));
    data["run_the_tests"] = json_string(t!("run_the_tests"));
    data["sqlx_cli"] = json_string(t!("sqlx_cli"));
    data["about_salvo"] = json_string(t!("about_salvo"));
    data["about_salvo_text"] = json_string(t!("about_salvo_text"));
    data["tip_title"] = json_string(t!("tip_title"));
    data["password_tip"] = json_string(t!("password_tip"));
    data["config_tip"] = json_string(t!("config_tip"));
    data["orm_title"] = json_string(t!("orm_title"));
    data["sqlx_website"] = json_string(t!("sqlx_website"));
    data["seaorm_website"] = json_string(t!("seaorm_website"));
    data["diesel_website"] = json_string(t!("diesel_website"));
    data["rbatis_website"] = json_string(t!("rbatis_website"));
    data["mongodb_website"] = json_string(t!("mongodb_website"));
    data["initialization"] = json_string(t!("initialization"));
    data["initialization_sqlx_cli_not_sqlite"] =
        json_string(t!("initialization_sqlx_cli_not_sqlite").replace(r"\n", "\n"));
    data["initialization_seaorm_cli_not_sqlite"] =
        json_string(t!("initialization_seaorm_cli_not_sqlite").replace(r"\n", "\n"));
    data["initialization_diesel_cli_not_sqlite"] =
        json_string(t!("initialization_diesel_cli_not_sqlite").replace(r"\n", "\n"));
    data["initialization_rbatis_cli_not_sqlite"] =
        json_string(t!("initialization_rbatis_cli_not_sqlite").replace(r"\n", "\n"));
    data["seaorm_cli_website"] = json_string(t!("seaorm_cli_website").replace(r"\n", "\n"));
    data["diesel_cli_website"] = json_string(t!("diesel_cli_website").replace(r"\n", "\n"));
    data["mongodb_usage_import_user_data"] =
        json_string(t!("mongodb_usage_import_user_data").replace(r"\n", "\n"));

    create_files(project_path, &data)
}

fn json_string(value: impl Into<String>) -> JsonValue {
    JsonValue::String(value.into())
}

fn create_files(project_path: &Path, data: &serde_json::Value) -> Result<()> {
    create_dir_all(project_path)?;
    let src_path = project_path.join("src");
    create_dir_all(src_path)?;

    // Render the template

    // Get the parent directory of the file
    if let Some(parent) = file_path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }

    // Create the file and write the rendered template to it
    let mut file = fs::File::create(file_path)?;
    file.write_all(rendered.as_bytes())?;

    Ok(())
}

fn copy_binary_file<T: AsRef<Path>>(file_bytes: &[u8], target_path: T) -> std::io::Result<()> {
    // Ensure the target directory exists
    if let Some(parent) = target_path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }

    // Create the target file and write the source file bytes into it
    let mut target_file = File::create(target_path)?;
    target_file.write_all(file_bytes)
}

fn check_name(name: &str) -> Result<()> {
    restricted_names::validate_package_name(name, "package name")?;

    if restricted_names::is_keyword(name) {
        anyhow::bail!(t!("error_is_keyword", name = name));
    }
    if restricted_names::is_conflicting_artifact_name(name) {
        warning(t!("error_is_conflicting_artifact_name", name = name).replace(r"\n", "\n"));
    }
    if name == "test" {
        anyhow::bail!(t!("error_equal_test").replace(r"\n", "\n"))
    }
    if ["core", "std", "alloc", "proc_macro", "proc-macro"].contains(&name) {
        warning(t!("error_part_of_standard_library", name = name,).replace(r"\n", "\n"));
    }
    if restricted_names::is_windows_reserved(name) {
        if cfg!(windows) {
            anyhow::bail!(t!("error_is_windows_reserved", name = name),);
        } else {
            warning(t!("warning_is_windows_reserved", name = name).replace(r"\n", "\n"));
        }
    }
    if restricted_names::is_non_ascii_name(name) {
        warning(t!("warning_is_non_ascii_name", name = name).replace(r"\n", "\n"));
    }
    Ok(())
}
fn check_path(path: &Path) -> Result<()> {
    // warn if the path contains characters that will break `env::join_paths`
    if join_paths(slice::from_ref(&OsStr::new(path)), "").is_err() {
        let path = path.to_string_lossy();
        print_util::warning(t!("warning_invalid_path", path = path));
    }
    Ok(())
}

pub fn join_paths<T: AsRef<OsStr>>(paths: &[T], env: &str) -> Result<OsString> {
    env::join_paths(paths.iter()).with_context(|| {
        let mut message = t!("erroe_join_paths", env = env).replace(r"\n", "\n");
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
    fp_ignore_file.write_all(b"/target\n/migration/target")?;
    Ok(())
}

/// Equivalent to [`create_dir_all`] with better error messages.
pub fn create_dir_all(p: impl AsRef<Path>) -> Result<()> {
    _create_dir_all(p.as_ref())
}

fn _create_dir_all(p: &Path) -> Result<()> {
    fs::create_dir_all(p)
        .with_context(|| format!("failed to create directory `{}`", p.display()))?;
    Ok(())
}
