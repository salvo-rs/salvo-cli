use crate::Project;
use anyhow::{Context, Result};
use handlebars::Handlebars;
use print_util::success;
use rust_i18n::t;
use serde_json::json;
use std::{
    env,
    ffi::{OsStr, OsString},
    fs::{self, File},
    io::Write,
    path::Path,
    slice,
};

use super::{
    get_selection::{get_user_selected, DbConnectionType, DbType, TemplateType, UserSelected},
    print_util, restricted_names, warning,
};

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

            init_git(project_path)?;

            success(t!("create_success", project_name = project_name).replace(r"\n", "\n"));
        }
        None => anyhow::bail!("cli quit!"),
    }

    Ok(())
}

fn write_project_file(
    project_path: &Path,
    user_selected: UserSelected,
    project: Project,
) -> Result<()> {
    let handlebars = Handlebars::new();
    let is_web_site = user_selected.template_type == TemplateType::SalvoWebSite;
    let need_db_conn = user_selected.db_conn_type != DbConnectionType::Nothing;
    let is_sqlx = user_selected.db_conn_type == DbConnectionType::Sqlx;
    let is_mysql = user_selected.db_type == DbType::Mysql;
    let is_postgres = user_selected.db_type == DbType::Postgres;
    let is_sqlite = user_selected.db_type == DbType::Sqlite;
    let mut data = json!({
        "project_name": project.project_name,
        "dependencies": {
            "anyhow": "1.0.75",
            "clia-tracing-config": "0.2.5",
            "jsonwebtoken": "8.3.0",
            "once_cell": "1.18.0",
            "salvo": {
                "version": "*",
                "features": ["anyhow", "logging", "cors", "oapi", "jwt-auth", "rustls", "catch-panic"]
            },
            "serde": "1.0.188",
            "thiserror": "1.0.48",
            "time": "0.3.28",
            "tokio": {
                "version": "1",
                "features": ["full"]
            },
            "toml": "0.8.0",
            "tracing": "0.1"
        },
        "is_web_site":is_web_site,
        "need_db_conn":need_db_conn,
        "is_sqlx":is_sqlx,
        "is_mysql":is_mysql,
        "is_postgres":is_postgres,
        "is_sqlite":is_sqlite,
        "main_log_message":t!("main_log_message"),
        "config_error_no_exits":t!("config_error_no_exits"),
        "config_error_read":t!("config_error_read"),
        "config_error_parse":t!("config_error_parse"),
        "config_error_read_failed":t!("config_error_read_failed"),
    });
    if is_sqlx {
        // Add sqlx dependencies
        let mut dependencies = data["dependencies"].clone();
        if is_mysql {
            dependencies["sqlx"] = json!({
                "version": "0.7",
                "features": ["runtime-tokio", "macros", "mysql"]
            });
        }
        if is_postgres {
            dependencies["sqlx"] = json!({
                "version": "0.7",
                "features": ["runtime-tokio", "macros", "postgres"]
            });
        }
        if is_sqlite {
            dependencies["sqlx"] = json!({
                "version": "0.7",
                "features": ["runtime-tokio", "macros", "sqlite"]
            });
        }
        //add uuid dependency
        dependencies["uuid"] = json!({
            "version": "1.4.1",
            "features": ["v4", "fast-rng", "macro-diagnostics"]
        });
        //add rand dependency
        dependencies["rand"] = json!({
            "version": "0.8.5",
        });
        //add argon2 dependency
        dependencies["argon2"] = json!({
            "version": "0.5.2",
        });
        data["dependencies"] = dependencies;
    }   

    std::fs::create_dir_all(project_path)?;

    let src_path = project_path.join("src");
    //src
    std::fs::create_dir_all(&src_path)?;
    //src/main.rs
    let main_file_path = src_path.join("main.rs");
    let main_template = include_str!("../template/src/main_template.hbs");
    let main_rendered = handlebars.render_template(main_template, &data)?;
    let mut main_file = File::create(main_file_path)?;
    main_file.write_all(main_rendered.as_bytes())?;
    //src/Cargo.toml
    let cargo_file_path = project_path.join("Cargo.toml");
    let cargo_template = include_str!("../template/src/cargo_template.hbs");
    let cargo_rendered = handlebars.render_template(cargo_template, &data)?;
    let mut cargo_file = File::create(cargo_file_path)?;
    cargo_file.write_all(cargo_rendered.as_bytes())?;
    //src/config.rs
    let config_template = include_str!("../template/src/config_template.hbs");
    let config_rendered = handlebars.render_template(config_template, &data)?;
    let mut config_file = File::create(src_path.join("config.rs"))?;
    config_file.write_all(config_rendered.as_bytes())?;
    //src/app_error.rs
    let app_error_template = include_str!("../template/src/app_error.hbs");
    let app_error_rendered = handlebars.render_template(app_error_template, &data)?;
    let mut app_error_file = File::create(src_path.join("app_error.rs"))?;
    app_error_file.write_all(app_error_rendered.as_bytes())?;
    if need_db_conn {
        //src/db.rs
        let db_template = include_str!("../template/src/db.hbs");
        let db_rendered = handlebars.render_template(db_template, &data)?;
        let mut db_file = File::create(src_path.join("db.rs"))?;
        db_file.write_all(db_rendered.as_bytes())?;
    }
    //src/app_response.rs
    let app_response_template = include_str!("../template/src/app_response.hbs");
    let app_response_rendered = handlebars.render_template(app_response_template, &data)?;
    let mut app_response_file = File::create(src_path.join("app_response.rs"))?;
    app_response_file.write_all(app_response_rendered.as_bytes())?;

    //src/middleware
    let middleware_path = src_path.join("middleware");
    std::fs::create_dir_all(&middleware_path)?;
    let jwt_bytes = include_bytes!("../template/src/middleware/jwt.rs");
    let mut jwt_file = File::create(middleware_path.join("jwt.rs"))?;
    jwt_file.write_all(jwt_bytes)?;
    //src/middleware/mod.rs
    let mod_bytes = include_bytes!("../template/src/middleware/mod.rs");
    let mut mod_file = File::create(middleware_path.join("mod.rs"))?;
    mod_file.write_all(mod_bytes)?;
    //src/middleware/handle404.rs
    let handle404_template = include_str!("../template/src/middleware/handle_404.hbs");
    let handle404_rendered = handlebars.render_template(handle404_template, &data)?;
    let mut handle404_file = File::create(middleware_path.join("handle_404.rs"))?;
    handle404_file.write_all(handle404_rendered.as_bytes())?;

    //config
    let config_path = project_path.join("config");
    std::fs::create_dir_all(&config_path)?;
    //config/config.toml
    let config_template = include_str!("../template/config/config.hbs");
    let config_toml_rendered = handlebars.render_template(config_template, &data)?;
    let mut config_file = File::create(config_path.join("config.toml"))?;
    config_file.write_all(config_toml_rendered.as_bytes())?;
    //config/certs
    let cert_path = config_path.join("certs");
    std::fs::create_dir_all(&cert_path)?;
    //config/certs/cert.pem
    let cert_template = include_str!("../template/config/certs/cert.pem");
    let mut cert_file = File::create(cert_path.join("cert.pem"))?;
    cert_file.write_all(cert_template.as_bytes())?;
    //config/certs/key.pem
    let key_path = cert_path.join("key.pem");
    let key_template = include_str!("../template/config/certs/key.pem");
    let mut key_file = File::create(key_path)?;
    key_file.write_all(key_template.as_bytes())?;
    if is_web_site {
        //templates
        let template_path = project_path.join("templates");
        std::fs::create_dir_all(&template_path)?;
        //template/hello.html
        let hello_html_template = include_bytes!("../template/templates/hello.html");
        let mut hello_html_file = File::create(template_path.join("hello.html"))?;
        hello_html_file.write_all(hello_html_template)?;
        //template/handle_404.html
        let handle_404_template = include_bytes!("../template/templates/404.html");
        let mut handle_404_file = File::create(template_path.join("handle_404.html"))?;
        handle_404_file.write_all(handle_404_template)?;
        if need_db_conn {
            //template/login.html
            let login_html_template = include_bytes!("../template/templates/login.html");
            let mut login_html_file = File::create(template_path.join("login.html"))?;
            login_html_file.write_all(login_html_template)?;
            //template/user_list_page
            let user_list_page_template =
                include_bytes!("../template/templates/user_list_page.html");
            let mut user_list_page_file = File::create(template_path.join("user_list_page.html"))?;
            user_list_page_file.write_all(user_list_page_template)?;
        }
    }
    //src/router
    let router_path = src_path.join("routers");
    std::fs::create_dir_all(&router_path)?;
    //src/router/mod.rs
    let router_mod_template = include_str!("../template/src/routers/mod.hbs");
    let router_mod_rendered = handlebars.render_template(router_mod_template, &data)?;
    let mut router_mod_file = File::create(router_path.join("mod.rs"))?;
    router_mod_file.write_all(router_mod_rendered.as_bytes())?;
    //src/router/demo.rs
    let router_demo_template = include_str!("../template/src/routers/demo.hbs");
    let router_demo_rendered = handlebars.render_template(router_demo_template, &data)?;
    let mut router_demo_file = File::create(router_path.join("demo.rs"))?;
    router_demo_file.write_all(router_demo_rendered.as_bytes())?;
    if need_db_conn {
        //src/router/user.rs
        let router_user_template = include_str!("../template/src/routers/user.hbs");
        let router_user_rendered = handlebars.render_template(router_user_template, &data)?;
        let mut router_user_file = File::create(router_path.join("user.rs"))?;
        router_user_file.write_all(router_user_rendered.as_bytes())?;
    }

    if need_db_conn {
        //src/services
        let services_path = src_path.join("services");
        std::fs::create_dir_all(&services_path)?;
        //src/services/mod.rs
        let services_mod_template = include_str!("../template/src/services/mod.hbs");
        let services_mod_rendered = handlebars.render_template(services_mod_template, &data)?;
        let mut services_mod_file = File::create(services_path.join("mod.rs"))?;
        services_mod_file.write_all(services_mod_rendered.as_bytes())?;
        //src/services/user.rs
        let services_user_template = include_str!("../template/src/services/user.hbs");
        let services_user_rendered = handlebars.render_template(services_user_template, &data)?;
        let mut services_user_file = File::create(services_path.join("user.rs"))?;
        services_user_file.write_all(services_user_rendered.as_bytes())?;
        //src/utils
        let utils_path = src_path.join("utils");
        std::fs::create_dir_all(&utils_path)?;
        //src/utils/mod.rs
        let utils_mod_template = include_str!("../template/src/utils/mod.hbs");
        let utils_mod_rendered = handlebars.render_template(utils_mod_template, &data)?;
        let mut utils_mod_file = File::create(utils_path.join("mod.rs"))?;
        utils_mod_file.write_all(utils_mod_rendered.as_bytes())?;

        //src/utils/rand_utils.rs
        let rand_utils_template = include_str!("../template/src/utils/rand_utils.hbs");
        let rand_utils_rendered = handlebars.render_template(rand_utils_template, &data)?;
        let mut rand_utils_file = File::create(utils_path.join("rand_utils.rs"))?;
        rand_utils_file.write_all(rand_utils_rendered.as_bytes())?;

        //src/dtos
        let dtos_path = src_path.join("dtos");
        std::fs::create_dir_all(&dtos_path)?;
        //src/dtos/mod.rs
        let dtos_mod_template = include_str!("../template/src/dtos/mod.hbs");
        let dtos_mod_rendered = handlebars.render_template(dtos_mod_template, &data)?;
        let mut dtos_mod_file = File::create(dtos_path.join("mod.rs"))?;
        dtos_mod_file.write_all(dtos_mod_rendered.as_bytes())?;

        //src/dtos/user.rs
        let dtos_user_template = include_str!("../template/src/dtos/user.hbs");
        let dtos_user_rendered = handlebars.render_template(dtos_user_template, &data)?;
        let mut dtos_user_file = File::create(dtos_path.join("user.rs"))?;
        dtos_user_file.write_all(dtos_user_rendered.as_bytes())?;

        //src/models
        let models_path = src_path.join("models");
        std::fs::create_dir_all(&models_path)?;
        //src/models/mod.rs
        let models_mod_template = include_str!("../template/src/models/mod.hbs");
        let models_mod_rendered = handlebars.render_template(models_mod_template, &data)?;
        let mut models_mod_file = File::create(models_path.join("mod.rs"))?;
        models_mod_file.write_all(models_mod_rendered.as_bytes())?;

        //src/models/user.rs
        let models_user_template = include_str!("../template/src/models/user.hbs");
        let models_user_rendered = handlebars.render_template(models_user_template, &data)?;
        let mut models_user_file = File::create(models_path.join("user.rs"))?;
        models_user_file.write_all(models_user_rendered.as_bytes())?;

        if is_sqlx {
            if is_sqlite {
                //data
                let data_path = project_path.join("data");
                std::fs::create_dir_all(&data_path)?;
                //data/demo.db
                let demo_db_bytes= include_bytes!("../template/data/demo.db");
                let mut demo_db_file = File::create(data_path.join("demo.db"))?;
            }
            //.env
            let env_template = include_str!("../template/.env.hbs");
            let env_rendered = handlebars.render_template(env_template, &data)?;
            let mut env_file = File::create(project_path.join(".env"))?;
            env_file.write_all(env_rendered.as_bytes())?;
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
    fp_ignore_file.write_all(b"/target\n")?;
    Ok(())
}

/// Equivalent to [`std::fs::create_dir_all`] with better error messages.
pub fn create_dir_all(p: impl AsRef<Path>) -> Result<()> {
    _create_dir_all(p.as_ref())
}

fn _create_dir_all(p: &Path) -> Result<()> {
    fs::create_dir_all(p)
        .with_context(|| format!("failed to create directory `{}`", p.display()))?;
    Ok(())
}
