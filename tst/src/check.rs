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
    let mut fns: Vec<String> = Vec::new();
    let mut called_fns: Vec<String> = Vec::new();
    let mut index = 1;

    for line in code.clone() {
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

        match parse_line(cd, &mut vrs, index, &mut fns, &mut called_fns) {
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

    // Final pass: Check for undefined function calls
    for call in called_fns {
        let call = call.trim_end_matches("()");
        if !fns.contains(&call.to_string()) {
            //println!("called func : {} | fns : {:?}",&call,&fns);
            eprintln!(
                "{}{}{}{}",
                "Error: ".red(),
                format!("Undefined function call '{}'", call).bold().red(),
                " found.".red(),
                format!(" Consider adding a function definition for '{}'", call)
                    .bold()
                    .red()
            );
            exit(-1);
        }
    }
}

fn parse_line(
    line: &str,
    vrs: &mut Vec<Var>,
    index: usize,
    fns: &mut Vec<String>,
    called_fns: &mut Vec<String>,
) -> Result<(), String> {
    if line.starts_with("echoln") && line.ends_with(")") {
        peln(line, vrs, index)
    } else if line.starts_with("may ") {
        pvr(line, vrs, index)
    } else if line.starts_with("#") {
        Ok(()) // Comment line
    } else if line.trim().replace(" ", "").is_empty() {
        Ok(())
    } else if line.trim().starts_with("ON ") && line.trim().ends_with("{}") {
        pef(line, vrs, index, fns)
    } else if line.trim().starts_with("ON ") && line.trim().ends_with("{") {
        pef(line, vrs, index, fns)
    } else if line.trim() == "}" {
        Ok(())
    } else {
        let mut iserr = true;
        for i in fns {
            let fncll = format!("{}()", i);
            if line.trim() == fncll {
                iserr = false;
            }
            //println!("grabbed func {} and gen func {}", fncll, line.trim());
        }
        if iserr {
            called_fns.push(line.trim().to_string());
            Ok(())
        } else {
            Ok(())
        }
    }
}

#[allow(dead_code, unused_assignments, unused_variables)]
fn pef(line: &str, vrs: &[Var], index: usize, fns: &mut Vec<String>) -> Result<(), String> {
    let (mut infnm, mut gsb, mut gmd) = (false, false, false);
    let mut curwrd = String::new();
    let ii: Vec<char> = line.trim().chars().collect();

    for i in ii {
        if !infnm {
            curwrd.push(i);
            if curwrd == "ON" {
                infnm = true;
                curwrd.clear();
            }
        } else if infnm && !gsb {
            if i == '(' {
                if gsb {
                    // Check if '(' already encountered
                    eprintln!(
                        "{}{}{}{}",
                        "ERR - Unexpected '(' after '(': code(".red(),
                        index.to_string().red(),
                        ") -> ".red(),
                        line.bold().red()
                    );
                    exit(0);
                }
                gsb = true;
                curwrd = curwrd.trim().to_string();
                if curwrd.contains(" ") {
                    eprintln!(
                        "{}{}{}{}",
                        "ERR - Function Names Can't Have Whitespaces: code(".red(),
                        index.to_string().red(),
                        ") -> ".red(),
                        line.bold().red()
                    );
                    exit(0);
                }
                fns.push(curwrd.clone());
                curwrd.clear();
            } else if i == '{' {
                eprintln!(
                    "{}{}{}{}",
                    "ERR - Unexpected '{' before function signature complete: code(".red(),
                    index.to_string().red(),
                    ") -> ".red(),
                    line.bold().red()
                );
                exit(0);
            } else {
                curwrd.push(i);
            }
        } else if gsb && !gmd {
            if i == ')' {
                gmd = true;
            } else if i == '(' {
                eprintln!(
                    "{}{}{}{}",
                    "ERR - Unexpected '(' after '(': code(".red(),
                    index.to_string().red(),
                    ") -> ".red(),
                    line.bold().red()
                );
                exit(0);
            } else if i == '{' {
                eprintln!(
                    "{}{}{}{}",
                    "ERR - Unexpected '{' before closing ')': code(".red(),
                    index.to_string().red(),
                    ") -> ".red(),
                    line.bold().red()
                );
                exit(0);
            } else {
                curwrd.push(i);
            }
        } else if gmd {
            if i == '{' {
                break;
            } else if i == ')' {
                eprintln!(
                    "{}{}{}{}",
                    "ERR - Unexpected ')' after closing ')': code(".red(),
                    index.to_string().red(),
                    ") -> ".red(),
                    line.bold().red()
                );
                exit(0);
            } else if i == '(' {
                eprintln!(
                    "{}{}{}{}",
                    "ERR - Unexpected '(' after closing ')': code(".red(),
                    index.to_string().red(),
                    ") -> ".red(),
                    line.bold().red()
                );
                exit(0);
            } else if i != ' ' {
                eprintln!(
                    "{}{}{}{}",
                    "ERR - Unexpected characters after function signature: code(".red(),
                    index.to_string().red(),
                    ") -> ".red(),
                    line.bold().red()
                );
                exit(0);
            }
        }
    }

    if !gsb || !gmd {
        eprintln!(
            "{}{}{}{}",
            "ERR - Function signature incomplete: code(".red(),
            index.to_string().red(),
            ") -> ".red(),
            line.bold().red()
        );
        exit(0);
    }
    Ok(())
}

#[allow(dead_code, unused_assignments, unused_variables)]

fn peln(line: &str, vrs: &Vec<Var>, index: usize) -> Result<(), String> {
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
fn pvr(line: &str, vrs: &mut Vec<Var>, index: usize) -> Result<(), String> {
    let acd = &line[4..];
    let mut parts = acd.split('=').map(|s| s.trim());
    let name = parts
        .next()
        .ok_or_else(|| "Missing variable name".to_string())?;
    let val = parts
        .next()
        .ok_or_else(|| "Missing variable value".to_string())?;

    if val.starts_with('\"') && val.ends_with('\"') {
        vrs.push(Var {
            name: name.to_string(),
            val: val.to_string(),
            vt: VT::Str,
        });
    } else if val.parse::<f64>().is_ok() {
        if val.contains('.') {
            vrs.push(Var {
                name: name.to_string(),
                val: val.to_string(),
                vt: VT::Float,
            });
        } else if val.starts_with("\"") && val.ends_with("\"") {
            vrs.push(Var {
                name: name.to_string(),
                val: val.to_string(),
                vt: VT::Str,
            });
        } else if val.parse::<i128>().is_ok() {
            vrs.push(Var {
                name: name.to_string(),
                val: val.to_string(),
                vt: VT::Int,
            });
        } else if val.contains("+") || val.contains("-") || val.contains("/") || val.contains("*") {
            vrs.push(Var {
                name: name.to_string(),
                val: val.to_string(),
                vt: VT::RT,
            });
        }
    } else {
        return Err(format!("Unknown Type for: {}", line.bold().green()));
    }

    Ok(())
}
