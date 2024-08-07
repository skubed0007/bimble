mod check;
mod datagen;

use colored::Colorize;
use check::check;
use datagen::compile_project;
use std::{env::args, fs, path::Path, process::exit};

#[allow(unused_assignments)]

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        eprintln!(
            "{}",
            "Invalid project folder: Either no projects or more than one project. Please check."
                .bold()
                .red()
        );
        exit(-1);
    }
    let pf = &args[1];
    if !Path::try_exists(Path::new(pf)).unwrap() {
        eprintln!(
            "{}{}",
            "Invalid project folder: Please provide a valid folder -> ".red(),
            pf.bold().red()
        );
        exit(-1);
    } else {
        if !fs::metadata(pf).unwrap().is_dir() {
            eprintln!(
                "{}{}",
                "Invalid project folder: Please provide a valid folder -> ".red(),
                pf.bold().red()
            );
            exit(-1);
        }
    }
    let mf = format!("{}/main.bb", pf);
    if Path::try_exists(Path::new(&mf)).unwrap() {
        if !fs::metadata(&mf).unwrap().is_file() {
            eprintln!(
                "{}{}",
                "Unable to find 'main.bb' file at -> ".red(),
                mf.bold().red()
            );
            exit(-1);
        }
    } else {
        eprintln!(
            "{}{}",
            "Unable to find 'main.bb' file at -> ".red(),
            mf.bold().red()
        );
    }
    match fs::read_to_string(&mf) {
        Ok(code) => {
            let codes = code.split("\n");
            let mut cds: Vec<String> = Vec::new();
            for i in codes {
                cds.push(i.to_string());
            }
            check(cds);
            compile_project(pf.to_string());
        }
        Err(_) => {
            eprintln!("{}{}", "Error : Cannot Read Code From 'main.bb' at -> ", mf);
        }
    }
}
