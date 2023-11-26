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
    path::{Path, PathBuf},
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
                "version": "0.58",
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
        "user_list":t!("user_list"),
        "are_you_sure_you_want_to_delete":t!("are_you_sure_you_want_to_delete"),
        "page_not_found":t!("page_not_found"),
        "contact_support":t!("contact_support"),
        "return_to_homepage":t!("return_to_homepage"),
        "delete":t!("delete"),
        "yes":t!("yes"),
        "cancel":t!("cancel"),
        "operation":t!("operation"),
        "create_success_sea_orm__mysql_or_pgsql_install_sea_orm":t!("create_success_sea_orm__mysql_or_pgsql_install_sea_orm"),
        "create_success_mysql_or_pgsql_fist_use":t!("create_success_mysql_or_pgsql_fist_use").replace(r"\n", "\n"),
        "create_success_sea_orm__mysql_or_pgsql_fist_use":t!("create_success_sea_orm__mysql_or_pgsql_fist_use").replace(r"\n", "\n"),
        "create_success_diesel__mysql_or_pgsql_fist_use":t!("create_success_diesel__mysql_or_pgsql_fist_use").replace(r"\n", "\n"),
    });
    let mut dependencies = data["dependencies"].clone();
    handle_dependencies(
        &mut dependencies,
        need_db_conn,
        user_selected.db_type,
        user_selected.db_conn_type,
    );
    data["dependencies"] = dependencies;

    let (src_path, router_path) = create_basic_file(project_path, &handlebars, &data)?;
    //assets
    let assets_path = project_path.join("assets");
    std::fs::create_dir_all(&assets_path)?;
    //assets/favicon.ico
    let favicon_bytes = include_bytes!("../template/assets/favicon.ico");
    let mut favicon_file = File::create(assets_path.join("favicon.ico"))?;
    favicon_file.write_all(favicon_bytes)?;

    if is_web_site {
        //templates
        let template_path = project_path.join("templates");
        std::fs::create_dir_all(&template_path)?;

        //template/hello.html
        let hello_template = include_str!("../template/templates/hello.hbs");
        let mut hello_file = File::create(template_path.join("hello.html"))?;
        hello_file.write_all(hello_template.as_bytes())?;

        //template/handle_404.html
        let handle_404_template = include_str!("../template/templates/404.hbs");
        let handle_404_template_rendered =
            handlebars.render_template(handle_404_template, &data)?;
        let mut handle_404_file = File::create(template_path.join("handle_404.html"))?;
        handle_404_file.write_all(handle_404_template_rendered.as_bytes())?;
        //assets
        let assets_path = project_path.join("assets");
        std::fs::create_dir_all(&assets_path)?;

        //assets/favicon.ico
        let favicon_bytes = include_bytes!("../template/assets/favicon.ico");
        let mut favicon_file = File::create(assets_path.join("favicon.ico"))?;

        favicon_file.write_all(favicon_bytes)?;
        if need_db_conn {
            //assets/js
            let js_path = assets_path.join("js");
            std::fs::create_dir_all(&js_path)?;
            //assets/js/alpinejs.js
            let alpinejs_bytes = include_bytes!("../template/assets/js/alpinejs.js");
            let mut alpinejs_file = File::create(js_path.join("alpinejs.js"))?;
            alpinejs_file.write_all(alpinejs_bytes)?;
            //assets/js/sweetalert2.js
            let sweetalert2_bytes = include_bytes!("../template/assets/js/sweetalert2.js");
            let mut sweetalert2_file = File::create(js_path.join("sweetalert2.js"))?;
            sweetalert2_file.write_all(sweetalert2_bytes)?;
            //assets/js/tailwindcss.js
            let tailwindcss_bytes = include_bytes!("../template/assets/js/tailwindcss.js");
            let mut tailwindcss_file = File::create(js_path.join("tailwindcss.js"))?;
            tailwindcss_file.write_all(tailwindcss_bytes)?;
            //template/login.html
            let login_template = include_str!("../template/templates/login.hbs");
            let login_template_rendered = handlebars.render_template(login_template, &data)?;
            let mut login_file = File::create(template_path.join("login.html"))?;
            login_file.write_all(login_template_rendered.as_bytes())?;

            //template/user_list.html
            let user_list_template = include_str!("../template/templates/user_list.hbs");
            let user_list_template_rendered =
                handlebars.render_template(user_list_template, &data)?;
            let mut user_list_file = File::create(template_path.join("user_list.html"))?;
            user_list_file.write_all(
                user_list_template_rendered
                    .replace("[[", "{{")
                    .replace("]]", "}}")
                    .as_bytes(),
            )?;

            //template/user_list_page.html
            let user_list_page_template = include_str!("../template/templates/user_list_page.hbs");
            let user_list_page_template_rendered =
                handlebars.render_template(user_list_page_template, &data)?;
            let mut user_list_page_file = File::create(template_path.join("user_list_page.html"))?;
            user_list_page_file.write_all(user_list_page_template_rendered.as_bytes())?;
        }
    }
    if need_db_conn {
        //src/db.rs
        let db_template = include_str!("../template/src/db.hbs");
        let db_rendered = handlebars.render_template(db_template, &data)?;
        let mut db_file = File::create(src_path.join("db.rs"))?;
        db_file.write_all(db_rendered.as_bytes())?;

        //src/router/user.rs
        let router_user_template = include_str!("../template/src/routers/user.hbs");
        let router_user_rendered = handlebars.render_template(router_user_template, &data)?;
        let mut router_user_file = File::create(router_path.join("user.rs"))?;
        router_user_file.write_all(router_user_rendered.as_bytes())?;

        //src/router/static_routers.rs
        let router_static_routers_template =
            include_str!("../template/src/routers/static_routers.hbs");
        render_and_write_to_file(
            &handlebars,
            router_static_routers_template,
            &data,
            router_path.join("static_routers.rs"),
        )?;
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
        if is_sea_orm || is_sqlx {
            //src/entities
            let entities_path = src_path.join("entities");
            std::fs::create_dir_all(&entities_path)?;
            //src/entities/mod.rs
            let entities_mod_template = include_str!("../template/src/entities/mod.hbs");
            let entities_mod_rendered = handlebars.render_template(entities_mod_template, &data)?;
            let mut entities_mod_file = File::create(entities_path.join("mod.rs"))?;
            entities_mod_file.write_all(entities_mod_rendered.as_bytes())?;

            //src/entities/user.rs
            let entities_user_template = include_str!("../template/src/entities/user.hbs");
            let entities_user_rendered =
                handlebars.render_template(entities_user_template, &data)?;
            let mut entities_user_file = File::create(entities_path.join("user.rs"))?;
            entities_user_file.write_all(entities_user_rendered.as_bytes())?;
            if is_sea_orm {
                //src/entities/prelude.rs
                let entities_prelude_template =
                    include_str!("../template/src/entities/prelude.hbs");
                let entities_prelude_rendered =
                    handlebars.render_template(entities_prelude_template, &data)?;
                let mut entities_prelude_file = File::create(entities_path.join("prelude.rs"))?;
                entities_prelude_file.write_all(entities_prelude_rendered.as_bytes())?;
            }
            if is_sqlx {
                //data
                let data_path = project_path.join("data");
                std::fs::create_dir_all(&data_path)?;
                if is_sqlite {
                    //data/demo.db
                    let demo_db_bytes = include_bytes!("../template/data/demo.db");
                    let mut demo_db_file = File::create(data_path.join("demo.db"))?;
                    demo_db_file.write_all(demo_db_bytes)?;
                } else {
                    //data/init_sql.sql
                    let init_sql_templte = include_str!("../template/data/init_sql_sql.hbs");
                    let init_sql_rendered = handlebars.render_template(init_sql_templte, &data)?;
                    let mut init_sql_file = File::create(data_path.join("init_sql.sql"))?;
                    init_sql_file.write_all(init_sql_rendered.as_bytes())?;
                }
                //migrations
                let migrations_path: std::path::PathBuf = project_path.join("migrations");
                std::fs::create_dir_all(&migrations_path)?;
                //migrations/2021-10-20-000000_create_users_table/up.sql
                let up_sql_bytes =
                    include_bytes!("../template/migrations/20231001143156_users.sql");
                let mut up_sql_file =
                    File::create(migrations_path.join("20231001143156_users.sql"))?;
                up_sql_file.write_all(up_sql_bytes)?;
                //.env
                let env_template = include_str!("../template/.env.hbs");
                let env_rendered = handlebars.render_template(env_template, &data)?;
                let mut env_file = File::create(project_path.join(".env"))?;
                env_file.write_all(env_rendered.as_bytes())?;
            }
            if is_sea_orm {
                //migration
                let migration_path = project_path.join("migration");
                std::fs::create_dir_all(&migration_path)?;
                //migration/src
                let migration_src_path = migration_path.join("src");
                std::fs::create_dir_all(&migration_src_path)?;
                //migration/src/main.rs
                let migration_main_byetes = include_bytes!("../template/migration/src/main.rs");
                let mut migration_main_file = File::create(migration_src_path.join("main.rs"))?;
                migration_main_file.write_all(migration_main_byetes)?;
                //migration/src/lib.rs
                let migration_lib_byetes = include_bytes!("../template/migration/src/lib.rs");
                let mut migration_lib_file = File::create(migration_src_path.join("lib.rs"))?;
                migration_lib_file.write_all(migration_lib_byetes)?;
                //migration/src/m20220101_000001_create_table.rs
                let migration_create_table_byetes =
                    include_bytes!("../template/migration/src/m20220101_000001_create_table.rs");
                let mut migration_create_table_file =
                    File::create(migration_src_path.join("m20220101_000001_create_table.rs"))?;
                migration_create_table_file.write_all(migration_create_table_byetes)?;
                //migration/Cargo.toml
                let migration_cargo_template = include_str!("../template/migration/Cargo.toml.hbs");
                let migration_cargo_rendered =
                    handlebars.render_template(migration_cargo_template, &data)?;
                let mut migration_cargo_file = File::create(migration_path.join("Cargo.toml"))?;
                migration_cargo_file.write_all(migration_cargo_rendered.as_bytes())?;
                //migration/README.md
                let migration_readme_bytes = include_bytes!("../template/migration/README.md");
                let mut migration_readme_file = File::create(migration_path.join("README.md"))?;
                migration_readme_file.write_all(migration_readme_bytes)?;

                if is_sqlite {
                    //data
                    let data_path = project_path.join("data");
                    std::fs::create_dir_all(&data_path)?;
                    //data/demo.db
                    let demo_db_bytes = include_bytes!("../template/data/demo_sea_orm.db");
                    let mut demo_db_file = File::create(data_path.join("demo.db"))?;
                    demo_db_file.write_all(demo_db_bytes)?;
                } else {
                    let data_path = project_path.join("data");
                    std::fs::create_dir_all(&data_path)?;
                    //data/init_sql.sql
                    let init_sql_templte = include_str!("../template/data/init_sql_sql.hbs");
                    let init_sql_rendered = handlebars.render_template(init_sql_templte, &data)?;
                    let mut init_sql_file = File::create(data_path.join("init_sql.sql"))?;
                    init_sql_file.write_all(init_sql_rendered.as_bytes())?;
                }
                //.env
                let env_template = include_str!("../template/.env.hbs");
                let env_rendered = handlebars.render_template(env_template, &data)?;
                let mut env_file = File::create(project_path.join(".env"))?;
                env_file.write_all(env_rendered.as_bytes())?;
            }
        }
        if is_diesel {
            //src/schema.rs
            let schema_template = include_str!("../template/src/schema.hbs");
            let schema_rendered = handlebars.render_template(schema_template, &data)?;
            let mut schema_file = File::create(src_path.join("schema.rs"))?;
            schema_file.write_all(schema_rendered.as_bytes())?;
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
            //migrations
            let migrations_path: std::path::PathBuf = project_path.join("migrations");
            std::fs::create_dir_all(&migrations_path)?;
            //migrations/2023-10-21-084227_create_users_table
            let migrations_create_users_table_path =
                migrations_path.join("2023-10-21-084227_create_users_table");
            std::fs::create_dir_all(&migrations_create_users_table_path)?;
            //migrations/2023-10-21-084227_create_users_table/up.sql
            let up_sql_bytes = include_bytes!(
                "../template/diesel_migrations/2023-10-21-084227_create_users_table/up.sql"
            );
            let mut up_sql_file = File::create(migrations_create_users_table_path.join("up.sql"))?;
            up_sql_file.write_all(up_sql_bytes)?;
            //migrations/2023-10-21-084227_create_users_table/down.sql
            let down_sql_bytes = include_bytes!(
                "../template/diesel_migrations/2023-10-21-084227_create_users_table/down.sql"
            );
            let mut down_sql_file: File =
                File::create(migrations_create_users_table_path.join("down.sql"))?;
            down_sql_file.write_all(down_sql_bytes)?;
            //migrations/.keep
            let gitkeep_bytes: [u8; 0] = [];
            let mut gitkeep_file = File::create(migrations_path.join(".keep"))?;
            gitkeep_file.write_all(&gitkeep_bytes)?;
            //migrations/README.md
            let migration_readme_bytes = include_bytes!("../template/diesel_migrations/README.md");
            let mut migration_readme_file = File::create(migrations_path.join("README.md"))?;
            migration_readme_file.write_all(migration_readme_bytes)?;

            //.env
            let env_template = include_str!("../template/.env.hbs");
            let env_rendered = handlebars.render_template(env_template, &data)?;
            let mut env_file = File::create(project_path.join(".env"))?;
            env_file.write_all(env_rendered.as_bytes())?;

            //template//diesel.toml
            let diesel_template = include_str!("../template/diesel.hbs");
            let diesel_rendered = handlebars.render_template(diesel_template, &data)?;
            let mut diesel_file = File::create(project_path.join("diesel.toml"))?;
            diesel_file.write_all(diesel_rendered.as_bytes())?;

            //data
            let data_path = project_path.join("data");
            std::fs::create_dir_all(&data_path)?;
            //data/init_sql.sql
            let init_sql_templte = include_str!("../template/data/init_sql_sql.hbs");
            let init_sql_rendered = handlebars.render_template(init_sql_templte, &data)?;
            let mut init_sql_file = File::create(data_path.join("init_sql.sql"))?;
            init_sql_file.write_all(init_sql_rendered.as_bytes())?;
            if is_sqlite {
                //data/test.db
                let demo_db_bytes = include_bytes!("../template/data/diesel_test.db");
                let mut test_db_file = File::create(data_path.join("test.db"))?;
                test_db_file.write_all(demo_db_bytes)?;
            }
        }
        if is_rbatis {
            //src/entities
            let entities_path = src_path.join("entities");
            std::fs::create_dir_all(&entities_path)?;
            //src/entities/mod.rs
            let entities_mod_template = include_str!("../template/src/entities/mod.hbs");
            let entities_mod_rendered = handlebars.render_template(entities_mod_template, &data)?;
            let mut entities_mod_file = File::create(entities_path.join("mod.rs"))?;
            entities_mod_file.write_all(entities_mod_rendered.as_bytes())?;

            //src/entities/user.rs
            let entities_user_template = include_str!("../template/src/entities/user.hbs");
            let entities_user_rendered =
                handlebars.render_template(entities_user_template, &data)?;
            let mut entities_user_file = File::create(entities_path.join("user.rs"))?;
            entities_user_file.write_all(entities_user_rendered.as_bytes())?;

            //data
            let data_path = project_path.join("data");
            std::fs::create_dir_all(&data_path)?;
            match user_selected.db_type {
                DbType::Sqlite => {
                    //data/table_sqlite.sql
                    let table_sqlite_template = include_bytes!("../template/data/table_sqlite.sql");
                    let mut table_sqlite_file = File::create(data_path.join("table_sqlite.sql"))?;
                    table_sqlite_file.write_all(table_sqlite_template)?;
                }
                DbType::Mysql => {
                    //data/table_mysql.sql
                    let table_mysql_template = include_bytes!("../template/data/table_mysql.sql");
                    let mut table_mysql_file = File::create(data_path.join("table_mysql.sql"))?;
                    table_mysql_file.write_all(table_mysql_template)?;
                }
                DbType::Postgres => {
                    //data/table_postgres.sql
                    let table_postgres_template =
                        include_bytes!("../template/data/table_postgres.sql");
                    let mut table_postgres_file =
                        File::create(data_path.join("table_postgres.sql"))?;
                    table_postgres_file.write_all(table_postgres_template)?;
                }
                DbType::Mssql => {
                    //data/table_mssql.sql
                    let table_mssql_template = include_bytes!("../template/data/table_mssql.sql");
                    let mut table_mssql_file = File::create(data_path.join("table_mssql.sql"))?;
                    table_mssql_file.write_all(table_mssql_template)?;
                }
            }
        }
        if is_mongodb {
            //src/entities
            let entities_path = src_path.join("entities");
            std::fs::create_dir_all(&entities_path)?;
            //src/entities/mod.rs
            let entities_mod_template = include_str!("../template/src/entities/mod.hbs");
            let entities_mod_rendered = handlebars.render_template(entities_mod_template, &data)?;
            let mut entities_mod_file = File::create(entities_path.join("mod.rs"))?;
            entities_mod_file.write_all(entities_mod_rendered.as_bytes())?;

            //src/entities/user.rs
            let entities_user_template = include_str!("../template/src/entities/user.hbs");
            let entities_user_rendered =
                handlebars.render_template(entities_user_template, &data)?;
            let mut entities_user_file = File::create(entities_path.join("user.rs"))?;
            entities_user_file.write_all(entities_user_rendered.as_bytes())?;

            //data
            let data_path = project_path.join("data");
            std::fs::create_dir_all(&data_path)?;
            //data/users.json
            let users_json_bytes = include_bytes!("../template/data/users.json");
            let mut users_json_file = File::create(data_path.join("users.json"))?;
            users_json_file.write_all(users_json_bytes)?;
        }
    }
    Ok(())
}

fn create_basic_file(
    project_path: &Path,
    handlebars: &Handlebars<'_>,
    data: &serde_json::Value,
) -> Result<(PathBuf, PathBuf)> {
    std::fs::create_dir_all(project_path)?;
    let src_path = project_path.join("src");
    std::fs::create_dir_all(&src_path)?;

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
    let router_path = src_path.join("routers");
    std::fs::create_dir_all(&router_path)?;
    Ok((src_path, router_path))
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

/// Equivalent to [`std::fs::create_dir_all`] with better error messages.
pub fn create_dir_all(p: impl AsRef<Path>) -> Result<()> {
    _create_dir_all(p.as_ref())
}

fn _create_dir_all(p: &Path) -> Result<()> {
    fs::create_dir_all(p)
        .with_context(|| format!("failed to create directory `{}`", p.display()))?;
    Ok(())
}
