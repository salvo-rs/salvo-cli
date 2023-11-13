use anyhow::Result;
use dialoguer::{console::Style, theme::ColorfulTheme, Select};
use rust_i18n::t;

#[derive(Debug, Clone, Copy)]
pub struct UserSelected {
    pub template_type: TemplateType,
    pub db_type: DbType,
    pub db_conn_type: DbConnectionType,
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
        t!("salvo_web_site"),
        t!("salvo_web_api"),
        // "custom",
    ];
    let selection = Select::with_theme(&theme)
        .with_prompt(t!("welcome_message").replace(r"\n", "\n"))
        .default(0)
        .items(&selections[..])
        .interact()?;
    let template_type = match selection {
        0 => TemplateType::SalvoWebSite,
        1 => TemplateType::SalvoWebApi,
        _ => anyhow::bail!("Invalid selection"),
    };
    let db_conn_types = &[
        t!("db_conn_types_sqlx"),
        t!("db_conn_types_sea_orm"),
        t!("db_conn_types_diesel"),
        t!("db_conn_types_rbatis"),
        t!("db_conn_types_mongodb"),
        t!("db_conn_types_nothing"),
        // "custom",
    ];
    let db_conn_type_selection = Select::with_theme(&theme)
        .with_prompt(t!("select_db_conn_type").replace(r"\n", "\n"))
        .default(0)
        .items(&db_conn_types[..])
        .interact()?;
    let db_conn_type = match db_conn_type_selection {
        0 => DbConnectionType::Sqlx,
        1 => DbConnectionType::SeaOrm,
        2 => DbConnectionType::Diesel,
        3 => DbConnectionType::Rbatis,
        4 => DbConnectionType::Mongodb,
        5 => DbConnectionType::Nothing,
        _ => anyhow::bail!("Invalid db connection type selection"),
    };
    if db_conn_type == DbConnectionType::Nothing || db_conn_type == DbConnectionType::Mongodb {
        return Ok(Some(UserSelected {
            template_type,
            db_type: DbType::Sqlite,
            db_conn_type,
        }));
    }
    let mut db_types: Vec<&str> = vec!["sqlite", "mysql", "postgres"];
    if db_conn_type == DbConnectionType::Rbatis {
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
        template_type,
        db_type,
        db_conn_type,
    }))
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TemplateType {
    SalvoWebSite,
    SalvoWebApi,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DbType {
    Sqlite,
    Mysql,
    Postgres,
    Mssql,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DbConnectionType {
    Sqlx,
    SeaOrm,
    Diesel,
    Rbatis,
    Mongodb,
    Nothing,
}
