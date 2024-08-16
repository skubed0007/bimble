use colored::Colorize;
use std::{
    fs::{self, OpenOptions},
    io::{Seek, Write},
    path::Path,
    process::exit,
};

pub struct CompilerConfig {
    name: String,
    authors: String,
    version: String,
}

impl CompilerConfig {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            authors: String::new(),
            version: String::new(),
        }
    }
}

#[allow(unused_assignments, unused_variables)]
pub fn compile_project(project_path: String) {
    let source_file = format!("{}/{}", project_path, "main.bb");
    let config_file = format!("{}/{}", project_path, "cfg.bcf");
    let mut config = CompilerConfig::new();

    // Parse Compiler Configuration
    match fs::read_to_string(&config_file) {
        Ok(config_content) => {
            let config_lines: Vec<&str> = config_content.lines().collect();
            let mut parsed_config = CompilerConfig::new();
            for (line_number, line) in config_lines.iter().enumerate() {
                let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();

                if parts.len() > 2 {
                    eprintln!(
                        "{}{}{}",
                        "Error on line ".red(),
                        format!("{}", line_number + 1).bold().red(),
                        ": Too many values, expected only a single key-value pair.".red()
                    );
                    exit(-1);
                } else if parts.len() == 2 {
                    let key = parts[0].to_ascii_uppercase();
                    let value = parts[1].to_string();

                    match key.as_str() {
                        "NAME" => parsed_config.name = value,
                        "AUTHORS" => parsed_config.authors = value,
                        "VER" => parsed_config.version = value,
                        _ => {
                            eprintln!(
                                "{}{}{}{}{}",
                                "Error on line ".red(),
                                format!("{}", line_number + 1).bold().red(),
                                ": Invalid key '".red(),
                                key.bold().red(),
                                "'. Expected 'Name', 'Authors', or 'Ver'.".red()
                            );
                            exit(-1);
                        }
                    }
                } else if parts.len() == 1 && !parts[0].is_empty() {
                    eprintln!(
                        "{}{}{}{}{}",
                        "Error on line ".red(),
                        format!("{}", line_number + 1).bold().red(),
                        ": Invalid term '".red(),
                        parts[0].bold().red(),
                        "'. Expected 'NAME', 'AUTHORS', or 'VERSION'.".red()
                    );
                    exit(-1);
                }
            }

            // Check if any of the required keys are missing
            if parsed_config.name.is_empty() {
                eprintln!("{}", "Error: Missing 'NAME' key in config file.".red());
                exit(-1);
            }
            if parsed_config.authors.is_empty() {
                eprintln!("{}", "Error: Missing 'AUTHORS' key in config file.".red());
                exit(-1);
            }
            if parsed_config.version.is_empty() {
                eprintln!("{}", "Error: Missing 'VERSION' key in config file.".red());
                exit(-1);
            }
            config = parsed_config;
        }
        Err(err) => {
            eprintln!(
                "{}{}{}{}",
                "Unable to read config file: ".red(),
                config_file.bold().red(),
                ". Please check if it exists and is accessible. Error details: ".red(),
                err.to_string().bold().red()
            );
            exit(-1);
        }
    }

    match fs::read_to_string(&source_file) {
        Ok(source_code) => {
            let bytecode = source_code.as_bytes().to_vec();
            let mut binary_code = String::new();
            for byte in bytecode {
                binary_code.push_str(&format!("{:08b}", byte).trim());
                binary_code.push('.');
            }
            let linux_vm = "./lb.bjb";
            let windows_vm = "./wb.bjb";
            let build_dir = format!("{}/build", project_path);

            if let Err(err) = clean_build_dir(&build_dir) {
                eprintln!("{}", err);
                exit(-1);
            }

            if let Err(err) = fs::create_dir(&build_dir) {
                eprintln!(
                    "{}{}{}",
                    "Unable to create build directory in project folder: ".red(),
                    project_path.bold().red(),
                    format!(". Error details: {}", err).red()
                );
                exit(-1);
            }

            // Compile for Linux and Windows platforms
            compile_binary("linux", &build_dir, &config, &binary_code, linux_vm);
            compile_binary("windows", &build_dir, &config, &binary_code, windows_vm);
        }
        Err(err) => {
            eprintln!(
                "{}{}{}",
                "Unable to read source file: ".red(),
                source_file.bold().red(),
                format!(". Error details: {}", err).red()
            );
            exit(-1);
        }
    }
}

fn clean_build_dir(build_dir: &str) -> Result<(), String> {
    if Path::new(build_dir).exists() {
        match fs::metadata(build_dir) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    if let Err(err) = fs::remove_dir_all(build_dir) {
                        return Err(format!(
                            "{}{}{}",
                            "Unable to delete build directory at: ".red(),
                            build_dir.bold().red(),
                            format!(". Error details: {}", err).red()
                        ));
                    }
                } else {
                    return Err(format!(
                        "{}{}",
                        "Build path exists but is not a directory: ".red(),
                        build_dir.bold().red()
                    ));
                }
            }
            Err(err) => {
                return Err(format!(
                    "{}{}{}",
                    "Unable to access build directory metadata at: ".red(),
                    build_dir.bold().red(),
                    format!(". Error details: {}", err).red()
                ));
            }
        }
    }
    Ok(())
}

fn compile_binary(
    platform: &str,
    build_dir: &str,
    config: &CompilerConfig,
    binary_code: &str,
    base_vm: &str,
) {
    let platform_dir = format!("{}/{}", build_dir, platform);
    let output_file = if platform == "linux" {
        format!("{}/{}", platform_dir, config.name)
    } else {
        format!("{}/{}.exe", platform_dir, config.name)
    };

    if let Err(err) = fs::create_dir(&platform_dir) {
        eprintln!(
            "{}{}{}",
            format!("Unable to create build directory for {}: ", platform).red(),
            platform_dir.bold().red(),
            format!(". Error details: {}", err).red()
        );
        return;
    }

    match fs::copy(base_vm, &output_file) {
        Ok(_) => {
            match OpenOptions::new()
                .append(true)
                .write(true)
                .open(&output_file)
            {
                Ok(mut exe) => {
                    let code_length = binary_code.chars().count();

                    // Append the binary code to the executable
                    if let Err(err) = exe.seek(std::io::SeekFrom::End(0)) {
                        eprintln!(
                            "{}{}{}",
                            format!("Unable to seek to the end of the {} executable: ", platform)
                                .red(),
                            output_file.bold().red(),
                            format!(". Error details: {}", err).red()
                        );
                        return;
                    }

                    if let Err(err) = exe.write_all(binary_code.as_bytes()) {
                        eprintln!(
                            "{}{}{}",
                            format!("Unable to write binary code to {} executable: ", platform)
                                .red(),
                            output_file.bold().red(),
                            format!(". Error details: {}", err).red()
                        );
                        return;
                    }

                    // Calculate the length of the appended data
                    let length_str = format!("{}", code_length);
                    let length_padding = 10;
                    let mut length_data = vec![b' '; length_padding];

                    // Pad the length string
                    let length_pos = length_padding - length_str.len();
                    length_data[length_pos..].copy_from_slice(length_str.as_bytes());

                    // Write the length at the end of the file
                    if let Err(err) = exe.seek(std::io::SeekFrom::End(-(length_padding as i64))) {
                        eprintln!(
                            "{}{}{}",
                            format!(
                                "Unable to seek to length padding position in {} executable: ",
                                platform
                            )
                            .red(),
                            output_file.bold().red(),
                            format!(". Error details: {}", err).red()
                        );
                        return;
                    }

                    if let Err(err) = exe.write_all(&length_data) {
                        eprintln!(
                            "{}{}{}",
                            format!(
                                "Unable to write length padding to {} executable: ",
                                platform
                            )
                            .red(),
                            output_file.bold().red(),
                            format!(". Error details: {}", err).red()
                        );
                        return;
                    }

                    println!(
                        "{}{}{}{}",
                        "Successfully compiled for ".blue(),
                        platform.bold().cyan(),
                        " at -> ".blue(),
                        platform_dir.cyan()
                    );
                }
                Err(err) => {
                    eprintln!(
                        "{}{}{}",
                        format!("Unable to generate {} executable at: ", platform).red(),
                        output_file.bold().red(),
                        format!(". Error details: {}", err).red()
                    );
                }
            }
        }
        Err(err) => {
            eprintln!(
                "{}{}{}",
                format!(
                    "Unable to copy base virtual machine (VM) for {} from {} to: ",
                    platform, base_vm
                )
                .red(),
                output_file.bold().red(),
                format!(". Error details: {}", err).red()
            );
        }
    }
}
