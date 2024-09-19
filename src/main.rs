use core::panic;
use std::io::ErrorKind;
use std::os::unix::fs;
use std::env;
use std::path::PathBuf;
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
    let target: PathBuf = args.target();

    if target.is_symlink() {
        match args.force {
            true => std::fs::remove_file(&target)?,
            false => panic!("Symlink already exists. Use flag -f or --force for force link."),
        }
    }

    eprintln!("target_path: {target:?}");
    match fs::symlink(file_name_abs, &target) {
        Ok(()) => (),
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => match args.force {
                false => panic!(""),
                // TODO: implement force link
                true => std::fs::remove_file(target)?
            }
            other_error => panic!("Error: cannot create symlink: {other_error:?}")
        }
    }
    Ok(())
}
