use ansi_term::Color;
use std::process::Command;
use std::str;

fn main() {
    let cow = Command::new("cowsay").arg("hello").output().unwrap();
    let cow = str::from_utf8(&cow.stdout).unwrap();
    println!("{}", Color::Yellow.bold().on(Color::Black).paint(cow));
}
