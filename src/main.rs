use std::error::Error;
use anstream::println;
use owo_colors::OwoColorize as _;
use clap::{Parser, Subcommand};
use lop::services::{vh7::Vh7Service, PasteService, Service, ShortenService};
use spinners::Spinner;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Shorten {
        url: String
    },
    Paste {
        code: String
    }
}

fn handle_error(error: Box<dyn Error>) {
    println!("{} {}", "Something has gone wrong:".red().bold(), error.to_string().red());
}

fn shorten(url: &str) -> Result<(), Box<dyn Error>> {
    let vh7 = Vh7Service::new()?;
    let mut sp = Spinner::new(spinners::Spinners::BouncingBar, "Shortening...".into());

    let res = vh7.shorten(url)?;

    sp.stop_and_persist("✔", "Done!".into());

    println!("{} {}", "Shortened:".green(), res.url.green().bold());

    if let Some(expiry) = res.expires {
        println!("  {} {}", "Expires".red(), expiry.format("%d %b %Y %H:%M").red());
    }

    Ok(())
}

fn paste(code: &str) -> Result<(), Box<dyn Error>> {
    let vh7 = Vh7Service::new()?;
    let mut sp = Spinner::new(spinners::Spinners::BouncingBar, "Pasting...".into());

    let res = vh7.paste(code, "")?;

    sp.stop_and_persist("✔", "Done!".into());

    println!("{} {}", "Pasted:".green(), res.url.green().bold());

    if let Some(expiry) = res.expires {
        println!("  {} {}", "Expires".red(), expiry.format("%d %b %Y %H:%M").red());
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Shorten { url }) => {
            if let Err(err) = shorten(url) {
                handle_error(err);
            }

            return Ok(())
        },
        Some(Commands::Paste { code }) => {
            if let Err(err) = paste(code) {
                handle_error(err);
            }

            return Ok(())
        }
        None => {}
    }

    println!("What's going on here!");

    Ok(())
}
