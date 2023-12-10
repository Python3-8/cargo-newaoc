use clap::Parser;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

const MAIN_RS: &str = include_str!("main_rs_template.rs");

/// A program that facilitates creating Rust projects for Advent of Code solutions
#[derive(Parser, Debug)]
pub struct Args {
    /// Name of the project (will be run with `cargo new`)
    name: String,
}

pub fn run(args: Args) {
    println!("WARNING: NOT FULLY TESTED, SO USE WITH CAUTION");
    let Args { name } = args;
    let exec = &format!("cargo new {name}");
    let status = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", exec]).status()
    } else {
        Command::new("sh").args(["-c", exec]).status()
    };
    match status {
        Ok(exit_status) => println!("cargo finished with status {exit_status}"),
        Err(e) => return eprintln!("error: {e:?}; exiting"),
    }
    let project_dir = Path::new(&name);
    match env::set_current_dir(project_dir) {
        Ok(_) => println!("successfully changed cwd to {}", project_dir.display()),

        Err(e) => {
            return eprintln!(
                "failed to change directory to {}: {e:?}",
                project_dir.display()
            )
        }
    }
    match fs::write("src/main.rs", MAIN_RS) {
        Ok(_) => println!("successfully wrote to src/main.rs"),
        Err(e) => return eprintln!("failed to write to src/main.rs: {e:?}"),
    }
    println!(
        "change directory into {} and happy coding!",
        project_dir.display()
    );
}
