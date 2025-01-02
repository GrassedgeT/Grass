use std::process;
use clap::Args;
#[derive(Args, Debug)]
pub struct BuildArgs {
    /// build in release mode
    #[arg(short, long, default_value_t = false)]    
    release: bool,
}

impl BuildArgs {
    pub fn build(&self) {
        process::Command::new("cargo")
            .args(["build", "--package", "kernel"])
            .status()
            .expect("failed to build kernel");
    }
}

