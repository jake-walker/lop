use std::{error::Error, fs};
use anstream::println;
use cli_clipboard;
use dialoguer::{theme::ColorfulTheme, Confirm};
use owo_colors::OwoColorize as _;
use clap::{Parser, Subcommand};
use lop::services::{vh7::Vh7Service, PasteService, Service, ServiceResult, ShortenService};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, global = true, help = "Only show the final output")]
    quiet: bool
}

#[derive(Subcommand)]
enum Commands {
    Shorten {
        url: String
    },
    Paste {
        #[arg(short, long, help = "The path to a file to create a paste from.")]
        filename: Option<String>,

        #[arg(short, long, help = "A string to create a paste from.")]
        code: Option<String>,

        #[arg(short = 'y', help = "Skip confirmation before pasting from clipboard or file.")]
        force: bool
    },
}

fn handle_error(error: Box<dyn Error>) {
    println!("{} {}", "Something has gone wrong:".red().bold(), error.to_string().red());
}

fn print_result(result: &ServiceResult, quiet: bool) {
    if quiet {
        println!("{}", result.url);
        return;
    }

    println!("{}", result.url.green().bold());

    if let Some(expiry) = result.expires {
        println!("  {} {}", "Expires".red(), expiry.format("%d %b %Y %H:%M").red());
    }
}

fn shorten(url: &str, quiet: bool) -> Result<(), Box<dyn Error>> {
    let vh7 = Vh7Service::new()?;

    let res = vh7.shorten(url)?;

    print_result(&res, quiet);

    Ok(())
}

fn paste(code: &str, quiet: bool) -> Result<(), Box<dyn Error>> {
    let vh7 = Vh7Service::new()?;

    let res = vh7.paste(code, "")?;

    print_result(&res, quiet);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Shorten { url }) => {
            if let Err(err) = shorten(url, cli.quiet) {
                handle_error(err);
            }

            return Ok(())
        },
        Some(Commands::Paste { filename, code, force }) => {
            let content = {
                if let Some(filename) = filename {
                    fs::read_to_string(filename)?
                } else if let Some(code) = code {
                    code.to_string()
                } else {
                    cli_clipboard::get_contents()?
                }
            };

            if !force && code.is_none() {
                println!("{}\n{}", "You are about to send the following:".blue().bold(), content.blue().italic());
                let confirmation = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Do you want to continue?")
                    .interact()?;

                if !confirmation {
                    return Ok(());
                }
            }

            if let Err(err) = paste(&content, cli.quiet) {
                handle_error(err);
            }

            return Ok(())
        }
        None => {}
    }

    println!("What's going on here!");

    Ok(())
}
