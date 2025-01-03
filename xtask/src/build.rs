use std::{process, vec};
use clap::Args;
#[derive(Args, Debug)]
pub struct BuildArgs {
    /// build in release mode
    #[arg(short, long, default_value_t = false)]    
    release: bool,
}

impl BuildArgs {
    pub fn build(&self) {
        //common cargo args
        let mut args = vec![
            "rustc", "--package", "kernel",
            "--target", "riscv64gc-unknown-none-elf",
        ];

        if self.release {
            args.push("--release");
        }

        //rustc flags
        let rustc_args = vec![
            "--",
            //use custom linker script
            "-Clink-arg=-Tcore/kernel/src/linker.ld",
            //force enable frame pointers
            "-Cforce-frame-pointers=yes",
        ];

        args.extend(rustc_args);
        
        process::Command::new("cargo")
            .args(args)
            .status()
            .expect("failed to build kernel");
    }
}

