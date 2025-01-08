use anyhow::Result;
use dialoguer::{console::Style, theme::ColorfulTheme, Select};
use rust_i18n::t;

#[derive(Debug, Clone, Copy)]
pub struct UserSelected {
    pub code_gen: CodeGen,
    pub db_type: DbType,
    pub db_lib: DbLib,
}

pub fn get_user_selected() -> Result<Option<UserSelected>> {
    let theme = ColorfulTheme {
        defaults_style: Style::new().blue(),
        prompt_style: Style::new().green().bold(),
        active_item_style: Style::new().blue().bold(),
        values_style: Style::new().blue().dim(),
        ..ColorfulTheme::default()
    };
    let selections = &[
        t!("salvo_website"),
        t!("salvo_openapi"),
        // "custom",
    ];
    let selection = Select::with_theme(&theme)
        .with_prompt(t!("welcome_message").replace(r"\n", "\n"))
        .default(0)
        .items(&selections[..])
        .interact()?;
    let code_gen = if selection == 1 {
        CodeGen::WebSite
    } else {
        CodeGen::OpenApi
    };
    let db_libs = &[
        t!("db_lib_sqlx"),
        t!("db_lib_sea_orm"),
        t!("db_lib_diesel"),
        t!("db_lib_rbatis"),
        t!("db_lib_mongodb"),
        t!("db_lib_nothing"),
        // "custom",
    ];
    let db_lib_selection = Select::with_theme(&theme)
        .with_prompt(t!("select_db_lib").replace(r"\n", "\n"))
        .default(0)
        .items(&db_libs[..])
        .interact()?;
    let db_lib = match db_lib_selection {
        0 => DbLib::Sqlx,
        1 => DbLib::SeaOrm,
        2 => DbLib::Diesel,
        3 => DbLib::Rbatis,
        4 => DbLib::Mongodb,
        5 => DbLib::Nothing,
        _ => anyhow::bail!("Invalid db connection type selection"),
    };
    if db_lib == DbLib::Nothing || db_lib == DbLib::Mongodb {
        return Ok(Some(UserSelected {
            code_gen,
            db_type: DbType::Sqlite,
            db_lib,
        }));
    }
    let mut db_types: Vec<&str> = vec!["sqlite", "mysql", "postgres"];
    if db_lib == DbLib::Rbatis {
        db_types = vec!["sqlite", "mysql", "postgres", "mssql"];
    }
    let db_type_selection = Select::with_theme(&theme)
        .with_prompt(t!("select_db_type").replace(r"\n", "\n"))
        .default(0)
        .items(&db_types[..])
        .interact()?;
    let db_type = match db_type_selection {
        0 => DbType::Sqlite,
        1 => DbType::Mysql,
        2 => DbType::Postgres,
        3 => DbType::Mssql,
        _ => anyhow::bail!("Invalid db type selection"),
    };

    Ok(Some(UserSelected {
        code_gen,
        db_type,
        db_lib,
    }))
}

#[derive(Debug, PartialEq, Clone, Copy, strum::Display)]
pub enum DbType {
    #[strum(serialize = "sqlite")]
    Sqlite,
    #[strum(serialize = "mysql")]
    Mysql,
    #[strum(serialize = "postgres")]
    Postgres,
    #[strum(serialize = "mssql")]
    Mssql,
}

#[derive(Debug, PartialEq, Clone, Copy, strum::Display)]
pub enum DbLib {
    Sqlx,
    SeaOrm,
    Diesel,
    Rbatis,
    Mongodb,
    Nothing,
}

#[derive(Debug, PartialEq, Clone, Copy, strum::Display)]
pub enum CodeGen {
    WebSite,
    OpenApi,
}
