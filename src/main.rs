mod lints;
mod runner;

use clap::Parser;
use colored::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "soroban-lint",
    about = "Static analysis tool for Soroban smart contracts",
    version = "0.1.0"
)]
struct Cli {
    /// Path to the Soroban contract directory or file
    path: PathBuf,

    /// Show only warnings and errors (suppress info)
    #[arg(short, long)]
    quiet: bool,
}

fn main() {
    let cli = Cli::parse();

    println!("{}", "soroban-lint 🔍".bold().cyan());
    println!("Analyzing: {}\n", cli.path.display());

    let results = runner::run(&cli.path, cli.quiet);

    if results.is_empty() {
        println!("{}", "✅ No issues found!".green().bold());
    } else {
        let warnings = results.iter().filter(|r| r.severity == "warning").count();
        let infos = results.iter().filter(|r| r.severity == "info").count();
        println!(
            "\n{}",
            format!(
                "Found {} issues ({} warnings, {} info)",
                results.len(),
                warnings,
                infos
            )
            .yellow()
            .bold()
        );
        std::process::exit(1);
    }
}
