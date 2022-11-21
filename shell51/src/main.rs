use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::process::Command;

fn main() {

    loop {
        print!("51sh >");
        stdout().flush().ok();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        let mut child = Command::new(command).spawn().unwrap();

        child.wait().ok();
    }
}
