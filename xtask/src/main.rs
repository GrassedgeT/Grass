mod build;
mod qemu;
mod debug;

use qemu::QemuArgs;
use build::BuildArgs;
use debug::DebugArgs;
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
    Build(BuildArgs),
    /// run kernel in QEMU 
    Qemu(QemuArgs),
    /// use gdb to debug kernel which is runing in QEMU
    Debug(DebugArgs),
}

fn main() {
    #[allow(clippy::enmu_glob_use)]
    use Commands::*;
    match Cli::parse().command {
        Build(args) => args.build(),
        Qemu(args) =>  args.run(),
        Debug(args) => args.debug(),
    }
}