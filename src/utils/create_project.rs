use anyhow::{Context, Result};
use handlebars::Handlebars;
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
use crate::Project;
use print_util::success;

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
            after_print_info(project_name, config);
        }
        None => anyhow::bail!("cli quit!"),
    }
    Ok(())
}

fn after_print_info(project_name: &String, config: UserSelected) {
    println!(); // a new line

    // print success info
    success(t!("create_success", project_name = project_name).replace(r"\n", "\n"));

    println!(); // a new line

    match config.db_conn_type {
        DbConnectionType::Sqlx => {
            success(t!("create_success_sqlx").replace(r"\n", "\n"));
            match config.db_type {
                DbType::Sqlite => {
                    success(t!("create_success_sqlx_sqlite").replace(r"\n", "\n"));
                }
                _ => {
                    success(t!("create_success_mysql_or_pgsql").replace(r"\n", "\n"));
                }
            }
        }
        DbConnectionType::SeaOrm => {
            success(t!("create_success_sea_orm").replace(r"\n", "\n"));
            match config.db_type {
                DbType::Sqlite => {
                    success(t!("create_success_sqlx_sqlite").replace(r"\n", "\n"));
                }
                _ => {
                    success(t!("create_success_mysql_or_pgsql").replace(r"\n", "\n"));
                }
            }
        }
        DbConnectionType::Diesel => match config.db_type {
            DbType::Sqlite => {
                success(t!("create_success_sqlx_diesel").replace(r"\n", "\n"));
            }
            _ => {
                success(t!("create_success_mysql_or_pgsql").replace(r"\n", "\n"));
            }
        },
        DbConnectionType::Rbatis => match config.db_type {
            DbType::Mysql | DbType::Postgres | DbType::Mssql => {
                success(t!("create_success_rbatis").replace(r"\n", "\n"));
            }
            _ => {}
        },
        DbConnectionType::Mongodb => {
            success(t!("mongodb_usage_import_user_data").replace(r"\n", "\n"));
            success(t!("access_instructions").replace(r"\n", "\n"));
        }
        _ => {}
    }
}

pub fn write_project_file(
    project_path: &Path,
    user_selected: UserSelected,
    project: Project,
) -> Result<()> {
    let handlebars = Handlebars::new();
    let is_web_site = user_selected.template_type == TemplateType::SalvoWebSite;
    let need_db_conn = user_selected.db_conn_type != DbConnectionType::Nothing;
    let is_sqlx = user_selected.db_conn_type == DbConnectionType::Sqlx;
    let is_sea_orm = user_selected.db_conn_type == DbConnectionType::SeaOrm;
    let is_diesel = user_selected.db_conn_type == DbConnectionType::Diesel;
    let is_rbatis = user_selected.db_conn_type == DbConnectionType::Rbatis;
    let is_mongodb = user_selected.db_conn_type == DbConnectionType::Mongodb;
    let is_mysql = user_selected.db_type == DbType::Mysql;
    let is_postgres = user_selected.db_type == DbType::Postgres;
    let is_sqlite = user_selected.db_type == DbType::Sqlite;
    let is_mssql = user_selected.db_type == DbType::Mssql;
    let is_sea_orm_or_sqlx = is_sea_orm || is_sqlx;
    let mut data = json!({
        "project_name": project.project_name,
        "dependencies": {
            "anyhow": "1.0.75",
            "clia-tracing-config": "0.2.5",
            "jsonwebtoken": "8.3.0",
            "once_cell": "1.18.0",
            "salvo": {
                "version": "0.60",
                "features": ["anyhow", "logging", "cors", "oapi", "jwt-auth", "rustls", "catch-panic","cookie","serve-static"]
            },
            "serde": "1.0.188",
            "thiserror": "1.0.48",
            "time": "0.3.28",
            "rust-embed":"8.0.0",
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
        "is_mssql":is_mssql,
        "is_sea_orm":is_sea_orm,
        "is_sea_orm_or_sqlx":is_sea_orm_or_sqlx,
        "is_diesel":is_diesel,
        "is_rbatis":is_rbatis,
        "is_mongodb":is_mongodb,
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
        "swagger_api_page":t!("swagger_api_page"),
        "login_page":t!("login_page"),
        "operation":t!("operation"),
        "create_success_sea_orm__mysql_or_pgsql_install_sea_orm":t!("create_success_sea_orm__mysql_or_pgsql_install_sea_orm"),
        "create_success_mysql_or_pgsql_fist_use":t!("create_success_mysql_or_pgsql_fist_use").replace(r"\n", "\n"),
        "create_success_sea_orm__mysql_or_pgsql_fist_use":t!("create_success_sea_orm__mysql_or_pgsql_fist_use").replace(r"\n", "\n"),
        "create_success_diesel__mysql_or_pgsql_fist_use":t!("create_success_diesel__mysql_or_pgsql_fist_use").replace(r"\n", "\n"),
    });
    data["is_starting"] = handlebars::JsonValue::String(t!("is_starting"));
    data["listen_on"] = handlebars::JsonValue::String(t!("listen_on"));
    data["database_connection_failed"] =
        handlebars::JsonValue::String(t!("database_connection_failed"));
    data["user_does_not_exist"] = handlebars::JsonValue::String(t!("user_does_not_exist"));
    let mut dependencies = data["dependencies"].clone();
    handle_dependencies(
        &mut dependencies,
        need_db_conn,
        user_selected.db_type,
        user_selected.db_conn_type,
    );
    data["dependencies"] = dependencies;
    create_basic_file(project_path, &handlebars, &data)?;
    copy_binary_file(
        include_bytes!("../template/assets/favicon.ico"),
        project_path.join("assets/favicon.ico"),
    )?;
    let mut templates: Vec<(&str, &str)> = vec![];
    if is_web_site {
        //templates
        let template_path = project_path.join("templates");
        create_dir_all(template_path)?;
        copy_binary_file(
            include_bytes!("../template/templates/hello.hbs"),
            project_path.join("templates/hello.html"),
        )?;
        let mut web_comm_templates = vec![(
            "templates/handle_404.html",
            include_str!("../template/templates/404.hbs"),
        )];
        templates.append(&mut web_comm_templates);
        if need_db_conn {
            copy_binary_file(
                include_bytes!("../template/assets/js/alpinejs.js"),
                project_path.join("assets/js/alpinejs.js"),
            )?;
            copy_binary_file(
                include_bytes!("../template/assets/js/sweetalert2.js"),
                project_path.join("assets/js/sweetalert2.js"),
            )?;
            copy_binary_file(
                include_bytes!("../template/assets/js/tailwindcss.js"),
                project_path.join("assets/js/tailwindcss.js"),
            )?;
            let mut web_db_templates = vec![
                (
                    "templates/login.html",
                    include_str!("../template/templates/login.hbs"),
                ),
                (
                    "templates/user_list.html",
                    include_str!("../template/templates/user_list.hbs"),
                ),
                (
                    "templates/user_list_page.html",
                    include_str!("../template/templates/user_list_page.hbs"),
                ),
            ];
            templates.append(&mut web_db_templates);
        }
    }
    if need_db_conn {
        let mut db_templates = vec![
            ("src/db.rs", include_str!("../template/src/db.hbs")),
            (
                "src/routers/user.rs",
                include_str!("../template/src/routers/user.hbs"),
            ),
            (
                "src/routers/static_routers.rs",
                include_str!("../template/src/routers/static_routers.hbs"),
            ),
            (
                "src/services/mod.rs",
                include_str!("../template/src/services/mod.hbs"),
            ),
            (
                "src/services/user.rs",
                include_str!("../template/src/services/user.hbs"),
            ),
            (
                "src/utils/mod.rs",
                include_str!("../template/src/utils/mod.hbs"),
            ),
            (
                "src/utils/rand_utils.rs",
                include_str!("../template/src/utils/rand_utils.hbs"),
            ),
            (
                "src/dtos/mod.rs",
                include_str!("../template/src/dtos/mod.hbs"),
            ),
            (
                "src/dtos/user.rs",
                include_str!("../template/src/dtos/user.hbs"),
            ),
        ];
        if is_sea_orm || is_sqlx {
            db_templates.append(
                vec![
                    (
                        "src/entities/mod.rs",
                        include_str!("../template/src/entities/mod.hbs"),
                    ),
                    (
                        "src/entities/users.rs",
                        include_str!("../template/src/entities/users.hbs"),
                    ),
                    (".env", include_str!("../template/.env.hbs")),
                ]
                .as_mut(),
            );
            if is_sea_orm {
                db_templates.push((
                    "src/entities/prelude.rs",
                    include_str!("../template/src/entities/prelude.hbs"),
                ));
            }
            if is_sqlx {
                //data
                let data_path = project_path.join("data");
                create_dir_all(data_path)?;
                if is_sqlite {
                    copy_binary_file(
                        include_bytes!("../template/data/demo.db"),
                        project_path.join("data/demo.db"),
                    )?;
                } else {
                    db_templates.push((
                        "data/init_sql.sql",
                        include_str!("../template/data/init_sql_sql.hbs"),
                    ));
                }
                copy_binary_file(
                    include_bytes!("../template/migrations/20231001143156_users.sql"),
                    project_path.join("migrations/2021-10-20-000000_create_users_table/up.sql"),
                )?;
            }
            if is_sea_orm {
                copy_binary_file(
                    include_bytes!("../template/migration/src/main.rs"),
                    project_path.join("migration/src/main.rs"),
                )?;
                copy_binary_file(
                    include_bytes!("../template/migration/src/lib.rs"),
                    project_path.join("migration/src/lib.rs"),
                )?;
                copy_binary_file(
                    include_bytes!("../template/migration/src/m20220101_000001_create_table.rs"),
                    project_path.join("migration/src/m20220101_000001_create_table.rs"),
                )?;
                copy_binary_file(
                    include_bytes!("../template/migration/README.md"),
                    project_path.join("migration/README.md"),
                )?;
                db_templates.append(
                    vec![(
                        "migration/Cargo.toml",
                        include_str!("../template/migration/Cargo.toml.hbs"),
                    )]
                    .as_mut(),
                );
                if is_sqlite {
                    copy_binary_file(
                        include_bytes!("../template/data/demo_sea_orm.db"),
                        project_path.join("data/demo.db"),
                    )?;
                } else {
                    db_templates.push((
                        "data/init_sql.sql",
                        include_str!("../template/data/init_sql_sql.hbs"),
                    ));
                }
            }
        }
        if is_diesel {
            db_templates.append(vec![
                (
                    "src/schema.rs",
                    include_str!("../template/src/schema.hbs"),
                ),
                (
                    "src/models/mod.rs",
                    include_str!("../template/src/models/mod.hbs"),
                ),
                (
                    "src/models/user.rs",
                    include_str!("../template/src/models/user.hbs"),
                ),
                (
                    "migrations/2023-10-21-084227_create_users_table/up.sql",
                    include_str!(
                        "../template/diesel_migrations/2023-10-21-084227_create_users_table/up.sql"
                    ),
                ),
                ("migrations/.keep","",),
                (
                    "migrations/2023-10-21-084227_create_users_table/down.sql",
                    include_str!(
                        "../template/diesel_migrations/2023-10-21-084227_create_users_table/down.sql"
                    ),
                ),
                (
                    "migrations/README.md",
                    include_str!("../template/diesel_migrations/README.md"),
                ),
                (".env", include_str!("../template/.env.hbs")),
                (
                    "template//diesel.toml",
                    include_str!("../template/diesel.hbs"),
                ),
                (
                    "data/init_sql.sql",
                    include_str!("../template/data/init_sql_sql.hbs"),
                ),
            ].as_mut());
            if is_sqlite {
                copy_binary_file(
                    include_bytes!("../template/data/diesel_test.db"),
                    project_path.join("data/test.db"),
                )?;
            }
        }
        if is_rbatis {
            db_templates.append(
                vec![
                    (
                        "src/entities/mod.rs",
                        include_str!("../template/src/entities/mod.hbs"),
                    ),
                    (
                        "src/entities/user.rs",
                        include_str!("../template/src/entities/users.hbs"),
                    ),
                ]
                .as_mut(),
            );

            match user_selected.db_type {
                DbType::Sqlite => {
                    copy_binary_file(
                        include_bytes!("../template/data/table_sqlite.sql"),
                        project_path.join("data/table_sqlite.sql"),
                    )?;
                }
                DbType::Mysql => {
                    copy_binary_file(
                        include_bytes!("../template/data/table_mysql.sql"),
                        project_path.join("data/table_mysql.sql"),
                    )?;
                }
                DbType::Postgres => {
                    copy_binary_file(
                        include_bytes!("../template/data/table_postgres.sql"),
                        project_path.join("data/table_postgres.sql"),
                    )?;
                }
                DbType::Mssql => {
                    copy_binary_file(
                        include_bytes!("../template/data/table_mssql.sql"),
                        project_path.join("data/table_mssql.sql"),
                    )?;
                }
            }
        }
        if is_mongodb {
            db_templates.append(
                vec![
                    (
                        "src/entities/mod.rs",
                        include_str!("../template/src/entities/mod.hbs"),
                    ),
                    (
                        "src/entities/user.rs",
                        include_str!("../template/src/entities/users.hbs"),
                    ),
                ]
                .as_mut(),
            );
            copy_binary_file(
                include_bytes!("../template/data/users.json"),
                project_path.join("data/users.json"),
            )?;
        }
        templates.append(&mut db_templates);
    }
    for (file_name, template) in &templates {
        render_and_write_to_file(&handlebars, template, &data, project_path.join(file_name))?;
    }
    Ok(())
}

fn create_basic_file(
    project_path: &Path,
    handlebars: &Handlebars<'_>,
    data: &serde_json::Value,
) -> Result<()> {
    create_dir_all(project_path)?;
    let src_path = project_path.join("src");
    create_dir_all(src_path)?;

    let templates = [
        (
            "Cargo.toml",
            include_str!("../template/src/cargo_template.hbs"),
        ),
        //src
        (
            "src/main.rs",
            include_str!("../template/src/main_template.hbs"),
        ),
        (
            "src/config.rs",
            include_str!("../template/src/config_template.hbs"),
        ),
        (
            "src/app_error.rs",
            include_str!("../template/src/app_error.hbs"),
        ),
        (
            "src/app_response.rs",
            include_str!("../template/src/app_response.hbs"),
        ),
        //src/middleware
        (
            "src/middleware/jwt.rs",
            include_str!("../template/src/middleware/jwt.hbs"),
        ),
        (
            "src/middleware/mod.rs",
            include_str!("../template/src/middleware/mod.hbs"),
        ),
        (
            "src/middleware/handle_404.rs",
            include_str!("../template/src/middleware/handle_404.hbs"),
        ),
        (
            "src/middleware/cors.rs",
            include_str!("../template/src/middleware/cors.hbs"),
        ),
        //config
        (
            "config/config.toml",
            include_str!("../template/config/config.hbs"),
        ),
        (
            "config/certs/cert.pem",
            include_str!("../template/config/certs/cert.pem"),
        ),
        (
            "config/certs/key.pem",
            include_str!("../template/config/certs/key.pem"),
        ),
        //src/routers
        (
            "src/routers/mod.rs",
            include_str!("../template/src/routers/mod.hbs"),
        ),
        (
            "src/routers/demo.rs",
            include_str!("../template/src/routers/demo.hbs"),
        ),
        (
            "src/routers/static_routers.rs",
            include_str!("../template/src/routers/static_routers.hbs"),
        ),
        (
            "src/services/mod.rs",
            include_str!("../template/src/services/mod.hbs"),
        ),
        (
            "src/services/user.rs",
            include_str!("../template/src/services/user.hbs"),
        ),
        (
            "src/utils/mod.rs",
            include_str!("../template/src/utils/mod.hbs"),
        ),
        (
            "src/utils/rand_utils.rs",
            include_str!("../template/src/utils/rand_utils.hbs"),
        ),
        (
            "src/dtos/mod.rs",
            include_str!("../template/src/dtos/mod.hbs"),
        ),
        (
            "src/dtos/user.rs",
            include_str!("../template/src/dtos/user.hbs"),
        ),
    ];

    for (file_name, template) in &templates {
        render_and_write_to_file(handlebars, template, &data, project_path.join(file_name))?;
    }
    Ok(())
}

fn handle_dependencies(
    dependencies: &mut serde_json::Value,
    need_db_conn: bool,
    db_type: DbType,
    conn_type: DbConnectionType,
) {
    if need_db_conn {
        dependencies["validator"] = json!({
            "version": "0.16",
            "features": ["derive"]
        });
        match (conn_type, db_type) {
            (DbConnectionType::Sqlx, DbType::Mysql) => {
                dependencies["sqlx"] = json!({
                    "version": "0.7",
                    "features": ["runtime-tokio", "macros", "mysql"]
                });
            }
            (DbConnectionType::Sqlx, DbType::Postgres) => {
                dependencies["sqlx"] = json!({
                    "version": "0.7",
                    "features": ["runtime-tokio", "macros", "postgres"]
                });
            }
            (DbConnectionType::Sqlx, DbType::Sqlite) => {
                dependencies["sqlx"] = json!({
                    "version": "0.7",
                    "features": ["runtime-tokio", "macros", "sqlite"]
                });
            }
            (DbConnectionType::SeaOrm, DbType::Mysql) => {
                dependencies["sea-orm"] = json!({
                    "version": "0",
                    "features": ["runtime-tokio-native-tls","sqlx-mysql"]
                });
            }
            (DbConnectionType::SeaOrm, DbType::Postgres) => {
                dependencies["sea-orm"] = json!({
                    "version": "0",
                    "features": ["runtime-tokio-native-tls","sqlx-postgres"]
                });
            }
            (DbConnectionType::SeaOrm, DbType::Sqlite) => {
                dependencies["sea-orm"] = json!({
                    "version": "0",
                    "features": ["runtime-tokio-native-tls","sqlx-sqlite"]
                });
            }
            (DbConnectionType::Diesel, DbType::Mysql) => {
                dependencies["diesel"] = json!({
                    "version": "2.1.0",
                    "features": ["mysql"]
                });
            }
            (DbConnectionType::Diesel, DbType::Postgres) => {
                dependencies["diesel"] = json!({
                    "version": "2.1.0",
                    "features": ["postgres"]
                });
            }
            (DbConnectionType::Diesel, DbType::Sqlite) => {
                dependencies["diesel"] = json!({
                    "version": "2.1.0",
                    "features": ["sqlite","returning_clauses_for_sqlite_3_35"]
                });
            }
            (DbConnectionType::Rbatis, DbType::Mysql) => {
                dependencies["rbs"] = json!({"version":"4.4"});
                dependencies["rbdc-mysql"] = json!({
                    "version": "4.4"
                });
                dependencies["rbatis"] = json!({
                    "version": "4.4",
                    "features": ["debug_mode"]
                });
            }
            (DbConnectionType::Rbatis, DbType::Postgres) => {
                dependencies["rbs"] = json!({"version":"4.4"});
                dependencies["rbdc-pg"] = json!({
                    "version": "4.4"
                });
                dependencies["rbatis"] = json!({
                    "version": "4.4",
                    "features": ["debug_mode"]
                });
            }
            (DbConnectionType::Rbatis, DbType::Sqlite) => {
                dependencies["rbs"] = json!({"version":"4.4"});
                dependencies["rbdc-sqlite"] = json!({
                    "version": "4.4"
                });
                dependencies["rbatis"] = json!({
                    "version": "4.4",
                    "features": ["debug_mode"]
                });
            }
            (DbConnectionType::Rbatis, DbType::Mssql) => {
                dependencies["rbs"] = json!({"version":"4.4"});
                dependencies["rbdc-mssql"] = json!({
                    "version": "4.4"
                });
                dependencies["rbatis"] = json!({
                    "version": "4.4",
                    "features": ["debug_mode"]
                });
            }
            (DbConnectionType::Mongodb, _) => {
                dependencies["mongodb"] = json!({"version":"2.0"});
                dependencies["futures-util"] = json!({
                    "version": "0.3",
                });
            }
            _ => {}
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
    }
}

fn render_and_write_to_file<T: AsRef<Path>>(
    handlebars: &Handlebars,
    template: &str,
    data: &impl serde::Serialize,
    file_path: T,
) -> Result<()> {
    // Render the template
    let rendered = handlebars.render_template(template, data)?;

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
