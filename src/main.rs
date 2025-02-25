use colored::Colorize;
use std::process::Command;
use std::str;

fn main() {
    let cow = Command::new("cowsay")
        .arg("hello")
        .output()
        .expect("Error running cowsay");
    let cow = str::from_utf8(&cow.stdout).unwrap();
    println!("{}", cow.bold().yellow().on_black());
}
