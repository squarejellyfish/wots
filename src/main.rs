use std::io::ErrorKind;
use std::os::unix::fs;
use std::env;
use std::path::PathBuf;
use clap::Parser;

/// A light-weight stow copy
#[derive(Parser, Debug)]
struct Cli {
    /// Path to the file to symlink
    file_name: PathBuf,
}

fn main() -> std::io::Result<()> {
    for arg in env::args() {
        println!("{arg}");
    }

    let args = Cli::parse();

    if !args.file_name.exists() {
        panic!("Error: file not exist: {:?}", args.file_name);
    }
    let file_name = args.file_name.file_name().unwrap();
    let home = env::var_os("HOME").unwrap();

    let target_path: PathBuf = PathBuf::from(home).join(file_name);
    let abs_file_name = std::path::absolute(&args.file_name).unwrap();

    println!("{target_path:?}");
    match fs::symlink(abs_file_name, target_path) {
        Ok(()) => (),
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => panic!("Symlink already exists."),
            other_error => panic!("Error: cannot create symlink: {other_error:?}")
        }
    }
    Ok(())
}
