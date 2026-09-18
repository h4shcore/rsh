use std::{io::{Write, stdin, stdout}, process::Command};

fn main() {
    loop {
        // use the '$ ' char as the prompt
        // need to explicitly flush this to ensure it prints before read_line
        print!("$ ");
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // everything after the first whitespace char
        // is interpreted as args to the command
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        let mut child = Command::new(command)
            .args(args)
            .spawn()
            .unwrap();

        // dont accept another command until this one completes
        child.wait();
    }
}
