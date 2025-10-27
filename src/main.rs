use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        match command {
            "exit" => {
                println!("Exiting shell...");
                break;
            }

            "cd" => {
                let path = args.get(0).map_or("/", |p| *p);

                if let Err(e) = std::env::set_current_dir(path) {
                    eprintln!("cd failed : {}", e);
                }
            }
            _ => {
                // Check for a pipe '|'
                if let Some(pipe_index) = args.iter().position(|&arg| arg == "|") {
                    // --- Handle Pipe ---

                    // 1. Split commands
                    let cmd1_args = &args[..pipe_index];
                    let cmd2_parts = &args[pipe_index + 1..];

                    if cmd2_parts.is_empty() {
                        eprintln!("Syntax error: No command provided after '|'");
                        continue;
                    }

                    let cmd2 = cmd2_parts[0];
                    let cmd2_args = &cmd2_parts[1..];

                    // 2. Spawn Command 1, capturing its stdout
                    let child1 = Command::new(command)
                        .args(cmd1_args)
                        .stdout(std::process::Stdio::piped()) // <- Key change!
                        .spawn();

                    let mut child1 = match child1 {
                        Ok(child) => child,
                        Err(e) => {
                            eprintln!("Failed to execute first command: {}", e);
                            continue;
                        }
                    };

                    // 3. Get the stdout handle from Command 1
                    // We use .take() to take ownership of the stdout handle
                    let child1_stdout = child1
                        .stdout
                        .take()
                        .expect("Failed to get stdout from first command");

                    // 4. Spawn Command 2, feeding it Command 1's stdout
                    let child2 = Command::new(cmd2)
                        .args(cmd2_args)
                        .stdin(std::process::Stdio::from(child1_stdout)) // <- Key change!
                        .spawn();

                    let mut child2 = match child2 {
                        Ok(child) => child,
                        Err(e) => {
                            eprintln!("Failed to execute second command: {}", e);
                            continue;
                        }
                    };

                    // 5. Wait for both commands to finish
                    child1.wait().expect("First command failed");
                    child2.wait().expect("Second command failed");
                } else {
                    // --- No Pipe: Handle Redirection (Your M4 code) ---
                    let mut output_file = None;
                    let mut command_parts: Vec<&str> = Vec::new();

                    if let Some(index) = args.iter().position(|&arg| arg == ">") {
                        if let Some(filename) = args.get(index + 1) {
                            match std::fs::File::create(filename) {
                                Ok(file) => output_file = Some(file),
                                Err(e) => {
                                    eprintln!("Failed to create file '{}': {}", filename, e);
                                    continue;
                                }
                            }
                        } else {
                            eprintln!("Syntax error: No filename provided after '>'");
                            continue;
                        }
                        command_parts.extend_from_slice(&args[..index]);
                    } else {
                        command_parts.extend_from_slice(&args);
                    }

                    let mut child_cmd = Command::new(command);
                    child_cmd.args(&command_parts);

                    if let Some(file) = output_file {
                        child_cmd.stdout(std::process::Stdio::from(file));
                    }

                    let child = child_cmd.spawn();
                    match child {
                        Ok(mut child) => {
                            child.wait().expect("command failed to run");
                        }
                        Err(e) => {
                            eprintln!("Failed to execute command: {}", e);
                        }
                    }
                }
            }
        }
    }
}
