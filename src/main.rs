// Copyright (c) 2024 Jake Walker
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{error::Error, fs};
use anstream::println;
use cli_clipboard;
use dialoguer::{theme::ColorfulTheme, Confirm};
use owo_colors::OwoColorize as _;
use clap::{Parser, Subcommand};
use lop::services::{vh7::Vh7Service, PasteService, Service, ServiceResult, ShortenService, UploadService};

#[derive(Parser)]
#[command(version, about, long_about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, global = true, help = "Only show the final output")]
    quiet: bool,

    #[arg(short = 'Q', long = "qr", global = true, help = "Show the output as a QR code")]
    qr_code: bool
}

#[derive(Subcommand)]
enum Commands {
    #[command(about="Shorten long URLs", long_about = None)]
    Shorten {
        url: String
    },
    #[command(about="Upload code", long_about = None)]
    Paste {
        #[arg(short, long, help = "The path to a file to create a paste from.")]
        filename: Option<String>,

        #[arg(short, long, help = "A string to create a paste from.")]
        code: Option<String>,

        #[arg(short = 'y', help = "Skip confirmation before pasting from clipboard or file.")]
        force: bool
    },
    #[command(about="Upload file")]
    Upload {
        filename: String
    }
}

fn handle_error(error: Box<dyn Error>) {
    println!("{} {}", "Something has gone wrong:".red().bold(), error.to_string().red());
}

fn print_result(result: &ServiceResult, quiet: bool, qr: bool) {
    if qr {
        if qr2term::print_qr(result.url.clone()).is_err() {
            println!("{}", "Failed to render QR code".red());
        };
    }

    if quiet && !qr {
        println!("{}", result.url);
        return;
    }

    println!("{}", result.url.green().bold());

    if let Some(expiry) = result.expires {
        println!("  {} {}", "Expires".red(), expiry.format("%d %b %Y %H:%M").red());
    }
}

fn shorten(srv: &impl ShortenService, url: &str, quiet: bool, qr: bool) -> Result<(), Box<dyn Error>> {
    let res = srv.shorten(url)?;

    print_result(&res, quiet, qr);

    Ok(())
}

fn paste(srv: &impl PasteService, code: &str, quiet: bool, qr: bool) -> Result<(), Box<dyn Error>> {
    let res = srv.paste(code, "")?;

    print_result(&res, quiet, qr);

    Ok(())
}

fn upload(srv: &impl UploadService, filename: &str, quiet: bool, qr: bool) -> Result<(), Box<dyn Error>> {
    let file = fs::read(filename)?;
    let res = srv.upload(file, filename.to_string(), "text/plain".to_string())?;

    print_result(&res, quiet, qr);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let srv = Vh7Service::new()?;

    match &cli.command {
        Commands::Shorten { url } => {
            if let Err(err) = shorten(&srv, url, cli.quiet, cli.qr_code) {
                handle_error(err);
            }
        },
        Commands::Paste { filename, code, force } => {
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

            if let Err(err) = paste(&srv, &content, cli.quiet, cli.qr_code) {
                handle_error(err);
            }
        },
        Commands::Upload { filename } => {
            if let Err(err) = upload(&srv, filename, cli.quiet, cli.qr_code) {
                handle_error(err);
            }
        }
    }

    Ok(())
}
