use std::io;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::{env, fs, string};
fn parse_command(command: String) -> Result<Vec<String>, String> {
    let return_vec: Vec<String> = command.split_whitespace().map(|s| s.to_string()).collect();
    if return_vec.len() < 2 {
        return Err("No Argument supplyed".to_string());
    }
    Ok(return_vec)
}
fn run_command(command: String) {
    let command = parse_command(command).unwrap_or(vec!["echo".to_string(), "Failed".to_string()]);
    let mut child = Command::new(&command[0])
        .args(&command[1..])
        .stdout(Stdio::piped())
        .spawn()
        .expect(&format!("Fail to exacute Command \n{}\n", command.concat()));

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        if let Ok(line) = line {
            println!("{}", line);
        }
    }

    let _ = child.wait();
}
fn convert_file_to_command(file_name: String) -> String {
    let content = fs::read_to_string(file_name).expect("Failed to Find file name");
    let parsed_content = parse_command(content).expect("Unable to unwarp content of file");
    println!("Parsed items count: {}", parsed_content.len());
    let test_string = parsed_content.join(" ");
    println!("{test_string}");
    test_string
}
pub fn run_file_command(file_name: String) {
    let mut path_file_name = String::from("/home/mckay/Coding/file_commands/");
    path_file_name.push_str(&file_name);
    path_file_name.push_str(".txt");
    run_command(convert_file_to_command(path_file_name));
}
