mod build;
use build::BuildArgs;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// build kernel and user Program
    Build(BuildArgs)
}

fn main() {
    #[allow(clippy::enmu_glob_use)]
    use Commands::*;
    match Cli::parse().command {
        Build(args) => {
            args.build();
        }
    }
}