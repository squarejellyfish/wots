use std::io::ErrorKind;
use std::env;
use std::path::{Path, PathBuf};
use clap::Parser;
use regex::Regex;

const HOME: &'static str = env!("HOME");

/// GNU stow, but much simpler and light-weighted
#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Path to the file to symlink, set to "." will link all the files in current directory.
    /// Respects .gitignore
    file_name: PathBuf,
    /// Target directory, default is home directory
    #[arg(short, long, value_name = "DIR", default_value = PathBuf::from(HOME).into_os_string())]
    target_path: PathBuf,
    /// Delete (unstow) the package from the target directory if this option is on
    #[arg(short, long, default_value_t = false)]
    delete: bool,
    /// Force the link(s) if already exists
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
        if args.delete {
            unimplemented!("Unlinking the whole directory is not implemented.")
        }
        link_whole_dir(&args)?;
        return Ok(())
    }

    let target: PathBuf = args.target();

    if args.delete {
        return delete_link(target)
    } 
    create_link(file_name_abs, target, args.force)
}

fn link_whole_dir(args: &Cli) -> std::io::Result<()> {
    let entries = std::fs::read_dir(".").expect("Error: cannot read the current directory");
    let ignore_files = get_ignore_files();

    // println!("Files to ignore:");
    // for file in &ignore_files {
    //     println!("\t{file:?}")
    // }

    for entry in entries {
        let entry = entry.expect("Error: cannot read directory").path();
        if !should_ignore(&entry, &ignore_files) {
            let entry_abs = std::path::absolute(&entry).unwrap();
            let target = PathBuf::from(HOME).join(entry.file_name().expect("Error: cannot get file name"));
            create_link(entry_abs, target, args.force)?;
        }
    }
    Ok(())
}

fn should_ignore(entry: &PathBuf, ignore_files: &[String]) -> bool {
    let entry_str = match entry.strip_prefix(".") {
        Ok(result) => result.to_str().unwrap(),
        Err(err) => panic!("Error: {err}: {entry:?}")
    };
    for ignore_file in ignore_files {
        let re = Regex::new(match ignore_file.strip_suffix('/') {
            Some(result) => result,
            None => ignore_file,
        }).unwrap_or_else(|e| {
            println!("Error: cannot parse {} to regex string: {}", ignore_file, e);
            panic!("This may cause unexpected behaviour of linking, aborting all operations.")
        });

        let result = re.is_match(entry_str);
        if result { return true }

        let entry_str_prefix = Path::new("/").join(entry_str);
        let result_prefix = re.is_match(entry_str_prefix.to_str().unwrap());

        if result_prefix { return true }
    }
    false
}

fn get_ignore_files() -> Vec<String> {
    let file_name = if Path::new("./.wots-ignore").exists() {
        Path::new("./.wots-ignore")
    } else if Path::new("~/.wots-global-ignore").exists() {
        Path::new("~/.wots-global-ignore")
    } else {
        panic!("Error: global wots ignore file does not exist");
    };
    let file_name_git = Path::new("./.gitignore");

    let mut ignore_files: Vec<String> = Vec::new();
    // get .gitignore files
    if file_name_git.exists() {
        let contents = std::fs::read_to_string(file_name_git).unwrap_or_else(|_| panic!("Error: cannot read file: {}", file_name_git.to_string_lossy()));
        for line in contents.lines() {
            if line.is_empty() {
                continue
            }
            ignore_files.push(line.to_string());
        }
    }

    // get .wots-ignore files
    let contents = std::fs::read_to_string(file_name).unwrap_or_else(|_| panic!("Error: cannot read wots-ignore file: {}", file_name.to_string_lossy()));
    for line in contents.lines() {
        if line.is_empty() {
            continue
        }
        ignore_files.push(line.to_string());
    }
    ignore_files.push(String::from(".wots-ignore"));
    ignore_files
}

fn create_link(file_name_abs: PathBuf, target: PathBuf, force: bool) -> std::io::Result<()> {
    if target.is_symlink() {
        match force {
            true => std::fs::remove_file(&target)?,
            false => panic!("Symlink already exists. Use flag -f or --force for force link."),
        }
    }

    println!("Linking {} to {}...", file_name_abs.to_str().unwrap(), target.to_str().unwrap());
    match std::os::unix::fs::symlink(file_name_abs, &target) {
        Ok(()) => Ok(()),
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => panic!("Error: cannot create symlink: {} already exists, and it's not a symlink", target.to_str().unwrap()),
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
