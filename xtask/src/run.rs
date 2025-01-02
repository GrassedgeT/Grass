use std::process;
use clap::Args;

#[derive(Args, Debug)]
pub struct RunArgs {
    
}

impl RunArgs {
    pub fn run(&self) {
        process::Command::new("cargo")
            .args(["run", "--package", "kernel"])
            .status()
            .expect("failed to run kernel");
    }
}
