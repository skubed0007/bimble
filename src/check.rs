use colored::Colorize;
use std::process::exit;

#[derive(Clone, Debug)]
#[allow(dead_code, unused_assignments, unused_variables)]
pub struct Var {
    name: String,
    val: String,
    vt: VT,
}

#[derive(Debug, Clone)]
pub enum VT {
    Str,
    Int,
    Float,
    RT,
}

pub fn check(code: Vec<String>) {
    let mut vrs: Vec<Var> = Vec::new();
    let mut index = 1;

    for line in code {
        let cd = line.trim();
        
        if cd.ends_with(';') {
            eprintln!(
                "{}{}{}{}{}",
                "Error: ".red(),
                "Unexpected semicolon".bold().red(),
                " at code (".red(),
                index.to_string().bold().red(),
                format!("): {}", cd).red().bold()
            );
            exit(-1);
        }

        match parse_line(cd, &mut vrs, index) {
            Ok(_) => (),
            Err(e) => {
                eprintln!(
                    "{}{}{}{}{}",
                    "Error: ".red(),
                    e.red(),
                    " at code (".red(),
                    index.to_string().bold().red(),
                    format!("): {}", cd).red().bold()
                );
                exit(-1);
            }
        }
        index += 1;
    }
}

fn parse_line(line: &str, vrs: &mut Vec<Var>, index: usize) -> Result<(), String> {
    if line.starts_with("echoln") && line.ends_with(")") {
        parse_echoln(line, vrs, index)
    } else if line.starts_with("may ") {
        parse_variable(line, vrs, index)
    } else if line.starts_with("#") {
        Ok(()) // Comment line
    } else if line.trim().replace(" ", "").is_empty() {
        Ok(())
    } else {
        Err(format!("Invalid syntax"))
    }
}

#[allow(dead_code, unused_assignments, unused_variables)]
fn parse_echoln(line: &str, vrs: &Vec<Var>, index: usize) -> Result<(), String> {
    let tcts = &line[7..line.len() - 1];
    let args = parse_arguments(tcts);

    for arg in &args {
        if !is_literal(arg) && !variable_exists(arg, vrs) {
            return Err(format!("Invalid argument to 'echoln' -> {}", arg));
        }
    }
    Ok(())
}

fn parse_arguments(tcts: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut curwrd = String::new();
    let mut intxt = false;
    let mut quote_char = ' ';

    for c in tcts.chars() {
        if intxt {
            curwrd.push(c);
            if c == quote_char {
                intxt = false;
            }
        } else {
            match c {
                '\"' | '\'' => {
                    intxt = true;
                    quote_char = c;
                    curwrd.push(c);
                }
                ',' => {
                    args.push(curwrd.trim().to_string());
                    curwrd.clear();
                }
                _ => curwrd.push(c),
            }
        }
    }

    if !curwrd.trim().is_empty() {
        args.push(curwrd.trim().to_string());
    }

    args
}

fn is_literal(arg: &str) -> bool {
    (arg.starts_with('"') && arg.ends_with('"')) || (arg.starts_with('\'') && arg.ends_with('\''))
}

fn variable_exists(arg: &str, vrs: &Vec<Var>) -> bool {
    vrs.iter().any(|var| var.name == *arg)
}

#[allow(dead_code, unused_assignments, unused_variables)]
fn parse_variable(line: &str, vrs: &mut Vec<Var>, index: usize) -> Result<(), String> {
    let acd = &line[4..];
    let mut parts = acd.split('=').map(|s| s.trim());

    let vn = parts.next().ok_or("Variable name missing")?.to_string();
    let vval = parts.next().ok_or("Variable value missing")?.to_string();

    if parts.next().is_some() {
        return Err("Too many '=' signs in variable declaration".to_string());
    }

    let vt = determine_variable_type(&vval, vrs)?;

    if vn.is_empty() {
        return Err("Variable name cannot be empty".to_string());
    }

    vrs.push(Var { name: vn, val: vval, vt });
    Ok(())
}

fn determine_variable_type(vval: &str, vrs: &Vec<Var>) -> Result<VT, String> {
    if is_literal(vval) {
        Ok(VT::Str)
    } else if vval.parse::<i128>().is_ok() {
        Ok(VT::Int)
    } else if vval.parse::<f64>().is_ok() {
        Ok(VT::Float)
    } else if vval.contains(&['+', '-', '/', '*'][..]) {
        Ok(VT::RT)
    } else {
        vrs.iter()
            .find(|var| var.name == *vval)
            .map(|var| var.vt.clone())
            .ok_or_else(|| format!("Unknown variable or type for value '{}'", vval))
    }
}
