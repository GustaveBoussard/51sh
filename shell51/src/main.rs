use std::env;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::process::{ Child, Command, Stdio };

fn main() {

    let mut cur_dir = env::current_dir().unwrap()
                                        .into_os_string()
                                        .into_string()
                                        .unwrap();
    let mut prev_dir = String::new();
    loop {
        print!("51sh > ");
        stdout().flush().ok();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut prev_cmd = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    let dir = args.peekable().peek().map_or("/", |x| *x);
                    match dir {
                        "-" => {
                            if let Err(e) = env::set_current_dir(&prev_dir) {
                                eprintln!("{}", e);
                            }
                            else {
                                prev_dir = String::from(cur_dir);
                                cur_dir = env::current_dir().unwrap()
                                        .into_os_string()
                                        .into_string()
                                        .unwrap();
                            }
                        },
                        dir => {
                            if let Err(e) = env::set_current_dir(&dir) {
                                eprintln!("{}", e);
                            }
                            else {
                                prev_dir = String::from(cur_dir);
                                cur_dir = String::from(dir);
                            }
                        }
                    }
                    prev_cmd = None;
                },
                "exit" => return,
                command => {
                    let stdin = prev_cmd.map_or(Stdio::inherit(),
                                |output: Child| Stdio::from(output.stdout
                                                                  .unwrap()));
                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => { prev_cmd = Some(output); },
                        Err(e) => {
                            prev_cmd = None;
                            eprintln!("{}", e);
                        },
                    };
                }
            }
        }

        if let Some(mut final_cmd) = prev_cmd {
            if let Err(e) = final_cmd.wait() {
                eprintln!("{}", e);
            }
        }
    }
}
