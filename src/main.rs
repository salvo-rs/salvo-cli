use anyhow::Result;
use clap::Parser;
use std::{net::IpAddr};
// use handlebars::Handlebars;
// use serde_json::json;
use dialoguer::{console::Style, theme::ColorfulTheme, Confirm, Input, Select};
mod utils;

#[derive(Parser, Debug)]
#[clap(version = "0.0.1", author = "Fankai liu <liufankai137@outlook.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    New(Project),
}
// get 子命令

#[derive(Parser, Debug)]
pub struct Project {
    pub project_name: String,
}
fn main() -> Result<()> {
    utils::print_logo();
    // match init_config() {
    //     Ok(None) => println!("Aborted."),
    //     Ok(Some(config)) => println!("{:#?}", config),
    //     Err(err) => println!("error: {}", err),
    // }

    // let selections = &[
    //     "Ice Cream",
    //     "Vanilla Cupcake",
    //     "Chocolate Muffin",
    //     "A Pile of sweet, sweet mustard",
    // ];

    // let selection = Select::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Pick your flavor")
    //     .default(0)
    //     .items(&selections[..])
    //     .interact()
    //     .unwrap();

    // println!("Enjoy your {}!", selections[selection]);

    // let selection = Select::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Optionally pick your flavor")
    //     .default(0)
    //     .items(&selections[..])
    //     .interact_opt()
    //     .unwrap();

    // if let Some(selection) = selection {
    //     println!("Enjoy your {}!", selections[selection]);
    // } else {
    //     println!("You didn't select anything!");
    // }

    // let selection = Select::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Optionally pick your flavor, hint it might be on the second page")
    //     .default(0)
    //     .max_length(2)
    //     .items(&selections[..])
    //     .interact()
    //     .unwrap();

    // println!("Enjoy your {}!", selections[selection]);

    // let mut reg = Handlebars::new();
    // // render without register
    // println!(
    //     "{}",
    //     reg.render_template("Hello {{name}}", &json!({"name": "foo"}))?
    // );
    // // register template using given name
    // reg.register_template_string("tpl_1", "Good afternoon, {{name}}")?;
    // println!("{}", reg.render("tpl_1", &json!({"name": "foo"}))?);
    let opts: Opts = Opts::parse();
    //dbg!(opts);
    match opts.subcmd {
        SubCommand::New(project) => {
            match utils::create_project(project)  {
                Ok(_) => (),
                Err(e) => utils::error(e.to_string()),
            };
        }
    }
    Ok(())
}
#[derive(Debug)]
#[allow(dead_code)]
struct Config {
    interface: IpAddr,
    hostname: String,
    use_acme: bool,
    private_key: Option<String>,
    cert: Option<String>,
}

fn init_config() -> Result<Option<Config>> {
    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };
    println!("Welcome to the setup wizard");
    if !Confirm::with_theme(&theme)
        .with_prompt("Do you want to continue?")
        .interact()?
    {
        return Ok(None);
    }

    let interface = Input::with_theme(&theme)
        .with_prompt("Interface")
        .default("127.0.0.1".parse().unwrap())
        .interact()?;

    let hostname = Input::with_theme(&theme)
        .with_prompt("Hostname")
        .interact()?;

    let tls = Select::with_theme(&theme)
        .with_prompt("Configure TLS")
        .default(0)
        .item("automatic with ACME")
        .item("manual")
        .item("no")
        .interact()?;

    let (private_key, cert, use_acme) = match tls {
        0 => (Some("acme.pkey".into()), Some("acme.cert".into()), true),
        1 => (
            Some(
                Input::with_theme(&theme)
                    .with_prompt("  Path to private key")
                    .interact()?,
            ),
            Some(
                Input::with_theme(&theme)
                    .with_prompt("  Path to certificate")
                    .interact()?,
            ),
            false,
        ),
        _ => (None, None, false),
    };

    Ok(Some(Config {
        hostname,
        interface,
        private_key,
        cert,
        use_acme,
    }))
}
