use anyhow::{Context, Result};
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Password};
use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use rustyline::{ColorMode, Editor};
use vpnutils::{Cli, CommandParser, Database};

fn run(db: vpnutils::Database, history_file: String) -> Result<()> {
    let mut rl = Editor::<()>::new();
    println!("Loading history from {}", history_file);
    if rl.load_history(&history_file).is_err() {
        // try saving it empty
        rl.save_history(&history_file)?;
    }
    rl.set_color_mode(ColorMode::Enabled);
    loop {
        let line = rl.readline(">> ");
        match line {
            Ok(line) => {
                let mut parsed_input = match shellwords::split(&line) {
                    Ok(a) => a,
                    Err(e) => {
                        println!("Error parsing input: {}", e);
                        continue;
                    }
                };
                parsed_input.insert(0, String::from("vpnutils"));
                let args = match CommandParser::try_parse_from(parsed_input) {
                    Ok(a) => a,
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                };
                // on successful parse, save the history to file we don't auto add to history
                // because we don't want to add invalid commands to the history file.
                // Unfortunately, this also exclude "help" commands from being added.
                rl.add_history_entry(line.as_str());
                rl.append_history(&history_file)?;
                let result = args.command.dispatch(&db);
                match result {
                    Ok(should_continue) => {
                        if should_continue {
                            continue;
                        }
                        break;
                    }
                    Err(e) => {
                        println!("Error: {e}");
                        continue;
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("bye");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }
    rl.append_history(&history_file)?;
    Ok(())
}

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let args = Cli::parse();
    // TODO implement password from stdin
    // TODO implement commands on commandline
    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Database Password")
        .interact()?;
    let xdg_dirs = xdg::BaseDirectories::with_prefix("vpnutils")?;
    let db = match Database::open(args.database_path.clone(), password.clone()) {
        Ok(db) => db,
        Err(e) => match e {
            vpnutils::DatabaseError::OpenError { source: _, path } => {
                println!("Database {} does not exist - creating", path);
                vpnutils::Database::create(args.database_path, password)
                    .context("cannot create database")?
            }
            vpnutils::DatabaseError::DecryptError(_) => {
                return Err(anyhow::anyhow!("Invalid password, cannot decrypt"));
            }
            _ => return Err(anyhow::anyhow!("Unknown error")),
        },
    };
    let history_filename = format!("history_{}.txt", str::replace(&db.path(), "/", "__"));
    let history_path = xdg_dirs.place_config_file(history_filename)?;
    let history_file = history_path
        .into_os_string()
        .into_string()
        .map_err(|err| anyhow::anyhow!("cannot part history path: {:?}", err))?;
    run(db, history_file)
}
