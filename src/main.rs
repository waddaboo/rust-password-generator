mod generate_password;
mod password_analyzer;
mod util;

use arboard::Clipboard;
use clap::{Parser, Subcommand};
use generate_password::{generate_password, generate_pin_number};
use human_panic::setup_panic;
use rand::{rngs::StdRng, thread_rng, RngCore, SeedableRng};
use util::{password_length_validator, pin_length_validator};

use crate::password_analyzer::PasswordAnalysis;

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(about = "A simple and lightweight password generator written in Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long)]
    analyze: bool,

    #[arg(long)]
    copy: bool,

    #[arg(long)]
    seed: Option<u64>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(name = "normal")]
    #[command(about = "Generate a random password with specified complexity")]
    #[command(
        long_about = "Generate a random password with options to include/exclude numbers and symbols for increased password complexity"
    )]
    Normal {
        #[arg(short, long, default_value = "12", value_parser = password_length_validator)]
        length: u32,

        #[arg(short, long)]
        numbers: bool,

        #[arg(short, long)]
        symbols: bool,
    },

    #[command(name = "pin")]
    #[command(about = "Generate a random PIN number")]
    #[command(
        long_about = "Generate a random PIN number with the option to configure the PIN number length"
    )]
    Pin {
        #[arg(short, long, default_value = "4", value_parser = pin_length_validator)]
        length: u32,
    },
}

fn main() {
    setup_panic!();

    let cli = Cli::parse();

    let mut rng: Box<dyn RngCore> = match cli.seed {
        Some(seed) => Box::new(StdRng::seed_from_u64(seed)),
        None => Box::new(thread_rng()),
    };

    let password = match cli.command {
        Commands::Normal {
            length,
            numbers,
            symbols,
        } => generate_password(&mut rng, length, numbers, symbols),

        Commands::Pin { length } => generate_pin_number(&mut rng, length),
    };

    if cli.copy {
        let mut clipboard = Clipboard::new().unwrap();
        clipboard
            .set_text(&password)
            .expect("Unable to access clipboard");
    }

    if cli.analyze {
        let analyzer = PasswordAnalysis::new(&password);
        analyzer.display_password_report();
    } else {
        println!("{}", password);
    }
}
