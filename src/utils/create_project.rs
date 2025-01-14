use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::{env, slice};

use anyhow::{Context, Result};
use liquid::model::{Object, Value};
use print_util::success;
use rust_i18n::t;

use super::get_selection::{get_user_selected, UserSelected};
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
    let mut data = liquid::object!({
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
    data["is_starting"] = Value::scalar(t!("is_starting").to_string());
    data["listen_on"] = Value::scalar(t!("listen_on").to_string());
    data["database_connection_failed"] = Value::scalar(t!("database_connection_failed").to_string());
    data["user_does_not_exist"] = Value::scalar(t!("user_does_not_exist").to_string());
    data["rust_version_tip"] = Value::scalar(t!("rust_version_tip").to_string());
    data["project_dir_description"] = Value::scalar(t!("project_dir_description").to_string());
    data["introduction"] = Value::scalar(t!("introduction").to_string());
    data["introduction_text"] = Value::scalar(t!("introduction_text").to_string());
    data["seleted_sqlite"] = Value::scalar(t!("seleted_sqlite").to_string());
    data["run_the_project"] = Value::scalar(t!("run_the_project").to_string());
    data["run_the_tests"] = Value::scalar(t!("run_the_tests").to_string());
    data["sqlx_cli"] = Value::scalar(t!("sqlx_cli").to_string());
    data["about_salvo"] = Value::scalar(t!("about_salvo").to_string());
    data["about_salvo_text"] = Value::scalar(t!("about_salvo_text").to_string());
    data["tip_title"] = Value::scalar(t!("tip_title").to_string());
    data["password_tip"] = Value::scalar(t!("password_tip").to_string());
    data["config_tip"] = Value::scalar(t!("config_tip").to_string());
    data["orm_title"] = Value::scalar(t!("orm_title").to_string());
    data["sqlx_website"] = Value::scalar(t!("sqlx_website").to_string());
    data["seaorm_website"] = Value::scalar(t!("seaorm_website").to_string());
    data["diesel_website"] = Value::scalar(t!("diesel_website").to_string());
    data["rbatis_website"] = Value::scalar(t!("rbatis_website").to_string());
    data["mongodb_website"] = Value::scalar(t!("mongodb_website").to_string());
    data["initialization"] = Value::scalar(t!("initialization").to_string());
    data["initialization_sqlx_cli_not_sqlite"] =
        Value::scalar(t!("initialization_sqlx_cli_not_sqlite").replace(r"\n", "\n"));
    data["initialization_seaorm_cli_not_sqlite"] =
        Value::scalar(t!("initialization_seaorm_cli_not_sqlite").replace(r"\n", "\n"));
    data["initialization_diesel_cli_not_sqlite"] =
        Value::scalar(t!("initialization_diesel_cli_not_sqlite").replace(r"\n", "\n"));
    data["initialization_rbatis_cli_not_sqlite"] =
        Value::scalar(t!("initialization_rbatis_cli_not_sqlite").replace(r"\n", "\n"));
    data["seaorm_cli_website"] = Value::scalar(t!("seaorm_cli_website").replace(r"\n", "\n"));
    data["diesel_cli_website"] = Value::scalar(t!("diesel_cli_website").replace(r"\n", "\n"));
    data["mongodb_usage_import_user_data"] =
        Value::scalar(t!("mongodb_usage_import_user_data").replace(r"\n", "\n"));

    create_files(project_path, &data)
}

fn create_files(project_path: &Path, data: &Object) -> Result<()> {
    for filename in Template::iter() {
        let file = Template::get(filename.as_ref()).expect("file must exist");

        let file_path = project_path.join(filename.as_ref());
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        if file_path.extension() == Some(OsStr::new("liquid")) {
            let template = liquid::ParserBuilder::with_stdlib()
                .build()
                .expect("should be valid template")
                .parse(&String::from_utf8_lossy(&file.data))?;
            let rendered = template.render(liquid::object!(data))?;
            let mut target_file = File::create(file_path)?;
            target_file.write_all(rendered.as_bytes())?;
        } else {
            let mut target_file = File::create(file_path)?;
            target_file.write_all(&file.data)?;
        }
    }

    Ok(())
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
