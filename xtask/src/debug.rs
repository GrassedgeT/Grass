use std::process;
use clap::Args;

#[derive(Args, Debug)]
pub struct DebugArgs {

}

impl DebugArgs {
    pub fn debug(&self) {
        let args = vec![
            "-ex", "file target/riscv64gc-unknown-none-elf/debug/kernel",
            "-ex", "set arch riscv:rv64",
            "-ex", "target remote :1234", 
        ];

        process::Command::new("riscv64-unknown-elf-gdb")
            .args(args)
            .status()
            .expect("failed to run kernel");
    }
}
