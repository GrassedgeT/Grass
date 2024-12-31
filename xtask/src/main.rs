mod build;

use std::{env};
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum XtaskCmd {
    build
}
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: XtaskCmd
}

fn main() {
}
