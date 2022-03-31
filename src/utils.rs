use std::fmt::Display;
use std::path::Path;

use clap::ValueHint;
use clap::{Arg, Command};

#[derive(Debug)]
pub enum TextStyles {
    Bold,
    Dim,
    Italic,
    Underline,
}

impl Display for TextStyles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_id = match self {
            TextStyles::Bold => "[1m",
            TextStyles::Dim => "[2m",
            TextStyles::Italic => "[3m",
            TextStyles::Underline => "[4m",
        };
        write!(f, "{}{}", '\u{001b}', type_id)
    }
}

pub enum Colors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Reset,
}

impl Display for Colors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_id = match self {
            Colors::Black => "[30m",
            Colors::Red => "[31m",
            Colors::Green => "[32m",
            Colors::Yellow => "[33m",
            Colors::Blue => "[34m",
            Colors::Magenta => "[35m",
            Colors::Cyan => "[36m",
            Colors::White => "[37m",
            Colors::Reset => "[0m",
        };
        write!(f, "{}{}", '\u{001b}', color_id)
    }
}

pub enum OsType {
    Linux,
    Mac,
    Windows,
}

pub fn render_error(error: &str) {
    println!(
        "{}[Error]{}: {}{}{}",
        Colors::Red,
        Colors::Reset,
        Colors::Cyan,
        error,
        Colors::Reset
    );
}

//Is first time
pub fn is_first_time() -> bool {
    !Path::new(&get_config_path()).exists()
}

/// Detecs os
pub fn detect_os() -> OsType {
    if cfg!(target_os = "linux") {
        OsType::Linux
    } else if cfg!(target_os = "macos") {
        OsType::Mac
    } else if cfg!(target_os = "windows") {
        OsType::Windows
    } else {
        unreachable!("Unsupported OS");
    }
}

pub fn render_info(info: &str) {
    println!(
        "{}[Info]{}: {}{}{}",
        Colors::Green,
        Colors::Reset,
        Colors::Cyan,
        info,
        Colors::Reset
    );
}

// Get env var
pub fn get_env_var(var: &str) -> Option<String> {
    match std::env::var(var) {
        Ok(val) => Some(val),
        Err(_) => None,
    }
}

// Get config path according to os
pub fn get_config_path() -> String {
    let os = detect_os();
    match os {
        OsType::Linux => {
            todo!()
        }
        OsType::Mac => match std::env::var("HOME") {
            Ok(path) => path,
            Err(_) => panic!("No home directory found"),
        },
        OsType::Windows => {
            todo!()
        }
    }
}

pub fn generate_lia_options() -> Command<'static> {
    Command::new("Lia")
        .about("Lia is the package manager and version manager for the Ellie Language")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("install")
                .about("Install a package")
                .arg(
                    Arg::new("jsonLog")
                        .help("Output json log")
                        .short('j')
                        .long("-json-log"),
                )
                .arg(
                    Arg::new("target")
                        .help("Target module to analyze")
                        .required(true)
                        .value_hint(ValueHint::FilePath),
                ),
        )
        .subcommand(
            Command::new("version")
                .about("Get version")
                .arg(
                    Arg::new("jsonLog")
                        .help("Output json log")
                        .short('j')
                        .long("-json-log"),
                )
                .arg(Arg::new("detailed").short('d').long("--detailed-version")),
        )
        .subcommand(
            Command::new("init").about("Initialize a new project").arg(
                Arg::new("path")
                    .help("Path of the project")
                    .short('p')
                    .long("--path"),
            ),
        )
}
