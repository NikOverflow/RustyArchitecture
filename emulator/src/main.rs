pub mod cpu;
pub mod opcodes;
#[cfg(test)]
pub mod tests;

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Shows Debug information.
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Binary File.
    #[arg(short, long)]
    file: String
}

fn main() {
    let args: Args = Args::parse();
    println!("Starting RustyArchitecture Emulator.");
    let mut cpu: cpu::CPU = cpu::CPU::new(args.verbose);
    cpu.load_from_file(&args.file);
    cpu.run();
}