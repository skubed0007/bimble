mod check;
mod datagen;

use colored::Colorize;
use check::check;
use datagen::compile_project;
use std::{env::args, fs, path::Path, process::exit};

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        eprintln!(
            "{}",
            "Error: Invalid number of arguments. Please provide exactly one project folder."
                .bold()
                .red()
        );
        exit(-1);
    }

    let pf = &args[1];
    if let Err(e) = validate_project_folder(pf) {
        eprintln!("{}", e);
        exit(-1);
    }

    let mf = format!("{}/main.bb", pf);
    if let Err(e) = validate_main_file(&mf) {
        eprintln!("{}", e);
        exit(-1);
    }

    match fs::read_to_string(&mf) {
        Ok(code) => {
            let cds: Vec<String> = code.lines().map(String::from).collect();
            check(cds);
            compile_project(pf.to_string());
        }
        Err(e) => {
            eprintln!(
                "{}{}{}: {}",
                "Error: ".bold().red(),
                "Cannot read code from 'main.bb' at -> ".bold().red(),
                mf.bold().red(),
                e.to_string().bold().red()
            );
            exit(-1);
        }
    }
}

fn validate_project_folder(pf: &str) -> Result<(), String> {
    let path = Path::new(pf);
    match Path::try_exists(path) {
        Ok(exists) => {
            if !exists {
                return Err(format!(
                    "{}{}",
                    "Error: Invalid project folder. Folder does not exist -> ".bold().red(),
                    pf.bold().red()
                ));
            }
        }
        Err(e) => {
            return Err(format!(
                "{}{}: {}",
                "Error: Failed to check if project folder exists -> ".bold().red(),
                pf.bold().red(),
                e.to_string().bold().red()
            ));
        }
    }

    match fs::metadata(pf) {
        Ok(metadata) => {
            if !metadata.is_dir() {
                return Err(format!(
                    "{}{}",
                    "Error: Invalid project folder. Not a directory -> ".bold().red(),
                    pf.bold().red()
                ));
            }
        }
        Err(e) => {
            return Err(format!(
                "{}{}: {}",
                "Error: Unable to access project folder -> ".bold().red(),
                pf.bold().red(),
                e.to_string().bold().red()
            ));
        }
    }

    Ok(())
}

fn validate_main_file(mf: &str) -> Result<(), String> {
    let path = Path::new(mf);
    match Path::try_exists(path) {
        Ok(exists) => {
            if !exists {
                return Err(format!(
                    "{}{}",
                    "Error: 'main.bb' file not found at -> ".bold().red(),
                    mf.bold().red()
                ));
            }
        }
        Err(e) => {
            return Err(format!(
                "{}{}: {}",
                "Error: Failed to check if 'main.bb' file exists -> ".bold().red(),
                mf.bold().red(),
                e.to_string().bold().red()
            ));
        }
    }

    match fs::metadata(mf) {
        Ok(metadata) => {
            if !metadata.is_file() {
                return Err(format!(
                    "{}{}",
                    "Error: 'main.bb' found, but it is not a file -> ".bold().red(),
                    mf.bold().red()
                ));
            }
        }
        Err(e) => {
            return Err(format!(
                "{}{}: {}",
                "Error: Unable to access 'main.bb' file -> ".bold().red(),
                mf.bold().red(),
                e.to_string().bold().red()
            ));
        }
    }

    Ok(())
}
