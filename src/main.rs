use run_command::run_file_command;
use std::{env, fs, string};
fn main() {
    // Skip args[0] (the executable path itself) and join remaining args with spaces
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: run_commands <command> [args...]");
        return;
    }

    let command_str = args.join(" ");
    run_file_command(command_str);
}