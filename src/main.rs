use core::panic;
use std::io::ErrorKind;
use std::env;
use std::path::{Path, PathBuf};
use clap::Parser;

const HOME: &'static str = env!("HOME");

/// A light-weight stow copy
#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Path to the file to symlink
    file_name: PathBuf,
    /// Target directory, default is home directory
    #[arg(short, long, value_name = "DIR", default_value = PathBuf::from(HOME).into_os_string())]
    target_path: PathBuf,
    /// Delete (unstow) the package from the target directory if this option is on
    #[arg(short, long, default_value_t = false)]
    delete: bool,
    /// Force the link if the link already exists
    #[arg(short, long, default_value_t = false)]
    force: bool,
}

impl Cli {
    fn file_name(&self) -> &std::ffi::OsStr {
        self.file_name.file_name().unwrap()
    }

    fn file_name_abs(&self) -> PathBuf {
        let ret = std::path::absolute(&self.file_name).unwrap();
        ret
    }

    fn target(&self) -> PathBuf {
        self.target_path.join(self.file_name())
    }
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    if !args.file_name.exists() {
        panic!("Error: file not exist: {:?}", args.file_name);
    }

    let file_name_abs = args.file_name_abs();
    // check if the file wotsing is the current_dir
    if file_name_abs == env::current_dir()? {
        link_whole_dir(args.force)?;
    }

    let target: PathBuf = args.target();

    if args.delete {
        return delete_link(target)
    } 
    create_link(file_name_abs, target, args.force)
}

fn link_whole_dir(force: bool) -> std::io::Result<()> {
    let entries = std::fs::read_dir(".")?;
    let ignore_files = get_ignore_files();
    println!("Files in current directory:");
    for entry in entries {
        println!("\t{entry:?}")
    }
    println!("Files to ignore:");
    for file in ignore_files {
        println!("\t{file:?}")
    }
    todo!("wotsing the whole directory is not yet implemented.")
}

fn get_ignore_files() -> Vec<String> {
    let file_name = if Path::new("./.wots-ignore").exists() {
        Path::new("./.wots-ignore")
    } else if Path::new("~/.wots-global-ignore").exists() {
        Path::new("~/.wots-global-ignore")
    } else {
        panic!("Error: global wots ignore file does not exist");
    };

    let contents = std::fs::read_to_string(file_name).unwrap_or_else(|_| panic!("Error: cannot read wots-ignore file: {}", file_name.to_string_lossy()));
    let mut ignore_files: Vec<String> = Vec::new();
    for line in contents.lines() {
        ignore_files.push(line.to_string());
    }
    ignore_files
}

fn create_link(file_name_abs: PathBuf, target: PathBuf, force: bool) -> std::io::Result<()> {
    if target.is_symlink() {
        match force {
            true => std::fs::remove_file(&target)?,
            false => panic!("Symlink already exists. Use flag -f or --force for force link."),
        }
    }

    eprintln!("target_path: {target:?}");
    match std::os::unix::fs::symlink(file_name_abs, &target) {
        Ok(()) => Ok(()),
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => unreachable!(),
            other_error => panic!("Error: cannot create symlink: {other_error:?}")
        }
    }
}

fn delete_link(target: PathBuf) -> std::io::Result<()> {
    match target.is_symlink() {
        true => std::fs::remove_file(&target),
        false => panic!("Error: Symlink does not exist: {target:?}"),
    }
}
