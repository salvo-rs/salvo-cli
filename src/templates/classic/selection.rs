use anyhow::Result;
use dialoguer::{console::Style, theme::ColorfulTheme, Select};
use rust_i18n::t;

#[derive(Debug, Clone, Copy)]
pub struct Selected {
    pub db_type: DbType,
    pub db_lib: DbLib,
}

pub fn get_selected() -> Result<Option<Selected>> {
    let theme = ColorfulTheme {
        defaults_style: Style::new().blue(),
        prompt_style: Style::new().green().bold(),
        active_item_style: Style::new().blue().bold(),
        values_style: Style::new().blue().dim(),
        ..ColorfulTheme::default()
    };
    // let selections = &[
    //     t!("salvo_website"),
    //     t!("salvo_openapi"),
    //     // "custom",
    // ];
    // let selection = Select::with_theme(&theme)
    //     .with_prompt(t!("welcome_message").replace(r"\n", "\n"))
    //     .default(0)
    //     .items(&selections[..])
    //     .interact()?;
    let db_libs = &[
        t!("db_lib_sqlx"),
        t!("db_lib_seaorm"),
        t!("db_lib_diesel"),
        t!("db_lib_rbatis"),
        t!("db_lib_mongodb"),
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
        _ => anyhow::bail!("Invalid db connection type selection"),
    };
    if db_lib == DbLib::Mongodb {
        return Ok(Some(Selected {
            db_type: DbType::Mongodb,
            db_lib,
        }));
    }

    let db_types = &[t!("db_type_sqlite"), t!("db_type_postgres"), t!("db_type_mysql")];
    let db_type_selection = Select::with_theme(&theme)
        .with_prompt(t!("select_db_type").replace(r"\n", "\n"))
        .default(0)
        .items(&db_types[..])
        .interact()?;
    let db_type = match db_type_selection {
        0 => DbType::Sqlite,
        1 => DbType::Postgres,
        2 => DbType::Mysql,
        _ => anyhow::bail!("Invalid db type selection"),
    };

    Ok(Some(Selected {
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
    Rbatis,
    #[strum(serialize = "mongodb")]
    Mongodb
}