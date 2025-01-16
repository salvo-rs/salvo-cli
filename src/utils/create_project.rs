use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::{env, slice};

use anyhow::{Context, Result};
use liquid::model::Object;
use print_util::success;
use rust_i18n::t;

use super::get_selection::{get_user_selected, UserSelected};
use super::{print_util, restricted_names, warning};
use crate::NewCmd;

#[derive(rust_embed::RustEmbed)]
#[folder = "./template"]
struct Template;

pub fn create_project(new_cmd: &NewCmd) -> Result<()> {
    check_name(&new_cmd.project_name)?;
    let project_name = &new_cmd.project_name;
    let project_path = Path::new(project_name);
    if project_path.exists() {
        anyhow::bail!(t!(
            "error_project_path_exist",
            path = project_path.to_string_lossy()
        ))
    }

    check_path(project_path)?;
    let Some(config) = get_user_selected()? else {
        anyhow::bail!("cli quit!")
    };
    match init_git(project_path) {
        Ok(_) => {}
        Err(e) => {
            warning(t!("warning_init_git", error = e).replace(r"\n", "\n"));
        }
    }

    create_files(project_path, config, new_cmd)?;

    after_print_info(project_name);
    Ok(())
}

fn after_print_info(project_name: &String) {
    println!(); // a new line
    success(t!("create_info", project_name = project_name).replace(r"\n", "\n"));
    success(t!("create_success").replace(r"\n", "\n"));
    success(t!("rust_version_tip"));
    println!(); // a new line
}

pub fn create_files(
    project_path: &Path,
    user_selected: UserSelected,
    new_cmd: &NewCmd,
) -> Result<()> {
    let code_gen = user_selected.code_gen.to_string();
    let db_lib = user_selected.db_lib.to_string();
    let db_type = user_selected.db_type.to_string();
    let data = liquid::object!({
        "project_name": new_cmd.project_name,
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
    "initialization_sqlx_cli_not_sqlite":
        t!("initialization_sqlx_cli_not_sqlite").replace(r"\n", "\n"),
    "initialization_seaorm_cli_not_sqlite":
        t!("initialization_seaorm_cli_not_sqlite").replace(r"\n", "\n"),
    "initialization_diesel_cli_not_sqlite":
        t!("initialization_diesel_cli_not_sqlite").replace(r"\n", "\n"),
    "initialization_rbatis_cli_not_sqlite":
        t!("initialization_rbatis_cli_not_sqlite").replace(r"\n", "\n"),
    "seaorm_cli_website": t!("seaorm_cli_website").replace(r"\n", "\n"),
    "diesel_cli_website": t!("diesel_cli_website").replace(r"\n", "\n"),
    "mongodb_usage_import_user_data":
       t!("mongodb_usage_import_user_data").replace(r"\n", "\n")
    });

    for filename in Template::iter() {
        if filename.starts_with("_base/") {
            let file = Template::get(filename.as_ref()).expect("file must exist");
            let file_path = project_path.join(filename.as_ref().trim_start_matches("_base/"));
            write_file(&file.data, &file_path, &data)?;
        } else if filename.starts_with("_data/") {
            if filename.contains(user_selected.db_lib.to_string().as_str())
                && filename.contains(user_selected.db_type.to_string().as_str())
            {
                let file = Template::get(filename.as_ref()).expect("file must exist");
                let file_path = project_path.join(filename.as_ref().trim_start_matches("_"));
                write_file(&file.data, &file_path, &data)?;
            }
        } else if filename.starts_with(user_selected.db_lib.to_string().as_str()) {
            let file = Template::get(filename.as_ref()).expect("file must exist");
            let file_path = project_path.join(
                filename
                    .as_ref()
                    .trim_start_matches(user_selected.db_lib.to_string().as_str()),
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
    let p = p.as_ref();
    fs::create_dir_all(p)
        .with_context(|| format!("failed to create directory `{}`", p.display()))?;
    Ok(())
}
