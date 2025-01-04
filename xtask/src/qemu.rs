use std::process;

use clap::Args;

#[derive(Args, Debug)]
pub struct QemuArgs {
    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

impl QemuArgs {
    pub fn run(&self) {
        let mut args = vec![
            "-machine",
            "virt",
            "-nographic",
            "-bios",
            "bootloader/rustsbi-qemu.bin",
            "-device",
            "loader,file=target/riscv64gc-unknown-none-elf/debug/kernel,addr=0x80200000",
        ];
        if self.debug {
            args.push("-s");
            args.push("-S");
        }
        process::Command::new("qemu-system-riscv64")
            .args(args)
            .status()
            .expect("failed to run kernel");
    }
}
