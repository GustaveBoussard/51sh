use std::env;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::process::Command;

fn main() {

    loop {
        print!("51sh > ");
        stdout().flush().ok();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                let dir = args.peekable().peek().map_or("/", |x| *x);
                if let Err(e) = env::set_current_dir(&dir) {
                    eprintln!("{}", e);
                }
            },
            command => {
                let mut child = Command::new(command).args(args).spawn().unwrap();
                child.wait().ok();
            }
        }
    }
}
