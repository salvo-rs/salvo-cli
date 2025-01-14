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
        t!("db_lib_seaorm"),
        t!("db_lib_diesel"),
        t!("db_lib_rbatis"),
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
        _ => anyhow::bail!("Invalid db connection type selection"),
    };
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
        3 => DbType::Mongodb,
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
    #[strum(serialize = "mongodb")]
    Mongodb
}

#[derive(Debug, PartialEq, Clone, Copy, strum::Display)]
pub enum DbLib {
    #[strum(serialize = "sqlx")]
    Sqlx,
    #[strum(serialize = "seaorm")]
    SeaOrm,
    #[strum(serialize = "diesel")]
    Diesel,
    #[strum(serialize = "rbatis")]
    Rbatis
}

#[derive(Debug, PartialEq, Clone, Copy, strum::Display)]
pub enum CodeGen {
    WebSite,
    OpenApi,
}
