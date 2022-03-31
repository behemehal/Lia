use crate::utils::{get_config_path, is_first_time, render_error, render_info};

pub mod utils;

use safe_en::{
    self,
    table::{Table, TableRow, TypeDefs},
};
use std::{fs, vec, path::Path};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\x1B]0;{}\x07", "Lia");

    if is_first_time() {
        render_info("First time, creating config file");

        //Create folder
        fs::create_dir(get_config_path() + "/.lia").unwrap_or_else(|e| {
            render_error(&format!(
                "Failed to create lia directory '{}'",
                e.raw_os_error().unwrap()
            ));
            std::process::exit(1)
        });

        //Create config file
        let mut db = safe_en::Database::new();
        db.set_name("Lia".to_string());

        //Installed Packages
        db.create_table(
            "packages",
            vec![
                TableRow::new("package_name".to_string(), TypeDefs::String),
                TableRow::new("vtersion".to_string(), TypeDefs::String),
                TableRow::new(
                    "dependencies".to_string(),
                    TypeDefs::Array(Box::new(TypeDefs::String)),
                ),
                TableRow::new("is_git".to_string(), TypeDefs::Bool),
                TableRow::new("git_url".to_string(), TypeDefs::String),
            ],
        )
        .unwrap();

        //Ellie Releases
        db.create_table(
            "ellie_releases",
            vec![
                TableRow::new("version".to_string(), TypeDefs::String),
                TableRow::new("url".to_string(), TypeDefs::String),
                TableRow::new("sha256".to_string(), TypeDefs::String),
                TableRow::new("size".to_string(), TypeDefs::String),
                TableRow::new("is_runtime".to_string(), TypeDefs::String),
                TableRow::new("is_compiler".to_string(), TypeDefs::String),
                TableRow::new("is_auto_complete".to_string(), TypeDefs::String),
            ],
        )
        .unwrap();

        //Config
        db.create_table(
            "config",
            vec![
                TableRow::new("auto_update".to_string(), TypeDefs::Bool),
                TableRow::new("db_version".to_string(), TypeDefs::I64),
                TableRow::new("app_version".to_string(), TypeDefs::String),
            ],
        )
        .unwrap();

        db.table("config")
            .unwrap()
            .insert(vec![true.into(), 0_i64.into(), "0.0.1".to_string().into()])
            .unwrap();

        db.save(&(get_config_path() + "/.lia/config.lia"));
        render_info("Config file created, installing ellie compiler and runtime");
    }

    let app = utils::generate_lia_options();
    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("init", matches)) => {
            let path_str = match matches.value_of("target") {
                Some(path) => path.to_string(),
                None => {
                    std::env::current_dir().unwrap().to_str().unwrap().to_string()
                }
            };

            if !Path::new(&path_str).exists() {
                render_error(&format!("Path '{}' not exists", &path_str));
                std::process::exit(1);
            } else if !Path::new(&path_str).is_dir() {
                render_error(&format!("Path '{}' is not a directory", &path_str));
                std::process::exit(1);
            } else if Path::new(&path_str).read_dir().unwrap().count() > 0 {
                render_error(&format!("Path '{}' is not empty", &path_str));
                std::process::exit(1);
            }
        }

        _ => unreachable!("clap should ensure we don't get here"),
    }

    println!("Hello, world!");
    Ok(())
}
 