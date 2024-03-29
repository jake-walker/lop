use std::error::Error;
use anstream::println;
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
        code: String
    }
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
        Some(Commands::Paste { code }) => {
            if let Err(err) = paste(code, cli.quiet) {
                handle_error(err);
            }

            return Ok(())
        }
        None => {}
    }

    println!("What's going on here!");

    Ok(())
}
