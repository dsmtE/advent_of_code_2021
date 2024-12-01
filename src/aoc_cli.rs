use crate::aoc_date::AocDate;
use crate::path::{get_input_path, get_module_name, get_module_path};

use std::fs::File;
use std::io::Write;
use std::process::Output;
use std::vec;

#[derive(Debug)]
pub enum AocCommandError {
    CommandNotFound,
    CommandNotCallable,
    BadExitStatus(Output),
}

impl std::fmt::Display for AocCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AocCommandError::CommandNotFound => write!(f, "aoc-cli is not present in environment."),
            AocCommandError::CommandNotCallable => write!(f, "aoc-cli could not be called."),
            AocCommandError::BadExitStatus(_) => {
                write!(f, "aoc-cli exited with a non-zero status.")
            }
        }
    }
}

pub fn check() -> Result<(), AocCommandError> {
    std::process::Command::new("aoc")
        .arg("-V")
        .output()
        .map_err(|_| AocCommandError::CommandNotFound)?;
    Ok(())
}

fn call_aoc_cli(args: &[String]) -> Result<Output, AocCommandError> {
    let output = std::process::Command::new("aoc")
        .args(args)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output()
        .map_err(|_| AocCommandError::CommandNotCallable)?;

    if output.status.success() {
        Ok(output)
    } else {
        Err(AocCommandError::BadExitStatus(output))
    }
}

fn build_date_args(aoc_date : &AocDate) -> Vec<String> {
    vec![
        "--day".into(),
        aoc_date.day.to_string(),
        "--year".into(),
        aoc_date.year.to_string()
    ]
}

pub fn read(aoc_date: &AocDate) -> Result<Output, AocCommandError> {
    let dir: std::path::PathBuf = std::env::temp_dir();
    let puzzle_path = dir.join(format!{"aoc-{}-{}.txt", aoc_date.year, aoc_date.day});
    
    let mut args: Vec<String> = build_date_args(aoc_date);

    args.append(&mut vec![
        "--puzzle-only".into(),
        "--puzzle-file".into(),
        puzzle_path.to_string_lossy().to_string(),
    ]);

    call_aoc_cli(&args)
}

pub fn calendar(aoc_date: &AocDate) -> Result<Output, AocCommandError> {
    call_aoc_cli(&[
        "calendar".into(),
        "--year".into(),
        aoc_date.year.to_string(),
    ])
}

pub fn download(aoc_date: &AocDate, overwrite: bool) -> Result<Output, AocCommandError> {
    let mut args: Vec<String> = build_date_args(aoc_date);

    let input_path = get_input_path(&aoc_date.day);

    args.append(&mut vec![
        "--input-only".into(),
        "--input-file".into(),
        input_path.clone(),
    ]);

    if overwrite {
        args.push("--overwrite".into());
    }

    args.push("download".into());

    let output = call_aoc_cli(&args)?;
    println!("🎄 Successfully wrote input to \"{}\".", &input_path);

    Ok(output)
}

const TEMPLATE_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/template.txt"));

fn safe_create_file(path: &str, overwrite: bool) -> Result<File, std::io::Error> {
    let mut file = std::fs::OpenOptions::new();
    if overwrite {
        file.create(true);
    } else {
        file.create_new(true);
    }
    file.truncate(true).write(true).open(path)
}

fn create_file(path: &str) -> Result<File, std::io::Error> {
    std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
}

pub fn template(get_aoc_date: &AocDate, download_input: bool, overwrite: bool) {
    // todo: handle download_input
    if download_input {
        println!("Not implemented yet.");
    }

    let day = get_aoc_date.day;

    let input_path = get_input_path(&day);
    let module_path = get_module_path(&day);

    let mut file = match safe_create_file(&module_path, overwrite) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create module file for day {day}: {e}");
            std::process::exit(1);
        }
    };

    match file.write_all(
        TEMPLATE_FILE
            .replace("%DAY_NUMBER%", &day.into_inner().to_string())
            .as_bytes(),
    ) {
        Ok(()) => {
            println!("Created module file \"{}\"", &module_path);
        }
        Err(e) => {
            eprintln!("Failed to write module contents: {e}");
            std::process::exit(1);
        }
    }

    match create_file(&input_path) {
        Ok(_) => {
            println!("Created empty input file \"{}\"", &input_path);
        }
        Err(e) => {
            eprintln!("Failed to create input file: {e}");
            std::process::exit(1);
        }
    }

    println!("---");
    println!("🎄 Type `cargo solve {day}` to run your solution.");
}

pub fn solve(aoc_date: &AocDate, release: bool) {
    let mut cmd_args = vec!["run".to_string()];
    
    if release {
        cmd_args.push("--release".to_string());
    }

    cmd_args.append(&mut vec![
        "--bin".to_string(),
        get_module_name(&aoc_date.day)
    ]);

    cmd_args.push("--".to_string());

    if  cfg!(debug_assertions) {
        println!("Running solution for day {} [cargo {}]", aoc_date.day, cmd_args.join(" "));
    }

    let mut cmd = std::process::Command::new("cargo")
        .args(&cmd_args)
        .spawn()
        .unwrap();

    cmd.wait().unwrap();
}