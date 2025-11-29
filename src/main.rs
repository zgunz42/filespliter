mod joiner;
mod splitter;

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "filesplitter")]
#[command(about = "Fast file splitter and joiner for RAR and other files", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Split a file into multiple parts")]
    Split {
        #[arg(short, long, help = "Input file to split")]
        input: PathBuf,

        #[arg(
            short,
            long,
            help = "Size of each part in bytes (e.g., 4294967296 for 4GB)"
        )]
        size: u64,
    },

    #[command(about = "Join part files back into original file")]
    Join {
        #[arg(short, long, help = "First part file (e.g., file.rar.part001)")]
        input: PathBuf,

        #[arg(short, long, help = "Output file path")]
        output: PathBuf,
    },
}

fn main() {
    print_banner();

    if let Err(e) = run() {
        eprintln!("\n{} {}", "✗ Error:".red().bold(), e.to_string().red());
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Split { input, size } => {
            handle_split(input, size)?;
        }
        Commands::Join { input, output } => {
            handle_join(input, output)?;
        }
    }

    Ok(())
}

fn print_banner() {
    println!(
        "\n{}",
        "╔═══════════════════════════════════════╗".bright_blue()
    );
    println!(
        "{}",
        "║       FILE SPLITTER & JOINER         ║"
            .bright_blue()
            .bold()
    );
    println!(
        "{}",
        "║         Fast • Efficient • Safe       ║".bright_blue()
    );
    println!(
        "{}",
        "╚═══════════════════════════════════════╝".bright_blue()
    );
}

fn handle_split(input: PathBuf, size: u64) -> Result<()> {
    let splitter = splitter::FileSplitter::new(input, size)?;
    splitter.split()?;
    Ok(())
}

fn handle_join(input: PathBuf, output: PathBuf) -> Result<()> {
    let joiner = joiner::FileJoiner::new(input, output)?;
    joiner.join()?;
    Ok(())
}
