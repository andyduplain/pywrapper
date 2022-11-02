use anyhow::{anyhow, Result};
use cfg_if::cfg_if;
use std::{env, fs, path::PathBuf, process::Command};

fn main() -> Result<()> {
    // We need the full path to the python executable for Command::new()
    let python_exe = find_python_exe()?;

    let mut args: Vec<String> = env::args().collect();

    // On Windows fs::canonicalize() will work if there is no path, which is the
    // intended way this exe will be invoked, however under Linux/macOS it will
    // fail, so we need to fallback to finding ourselves in the path.
    let exe = match fs::canonicalize(&args[0]) {
        Ok(exe) => exe,
        Err(_) => find_in_path(&args[0])?,
    };

    let exe_name = exe.file_name().unwrap().to_str().unwrap();
    let mut script_path = exe
        .clone()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("scripts")
        .join(exe_name);
    script_path.set_extension("py");

    if !script_path.is_file() {
        return Err(anyhow!(
            "The script '{}' does not exist!",
            script_path.to_str().unwrap()
        ));
    }

    // Substitute in the script
    args[0] = script_path.to_str().unwrap().to_string();

    Command::new(python_exe).args(args).status()?;

    Ok(())
}

fn find_python_exe() -> Result<PathBuf> {
    let exe_name;
    cfg_if! {
        if #[cfg(target_os = "windows")] {
            exe_name = "python.exe";
        } else if #[cfg(target_os = "linux")] {
            exe_name = "python3";
        } else if #[cfg(target_os = "macos")] {
            exe_name = "python3";
        } else {
            panic!("Unsupported platform");
        }
    }

    find_in_path(exe_name)
}

fn find_in_path(exe_name: &str) -> Result<PathBuf> {
    let path_delim;
    cfg_if! {
        if #[cfg(target_os = "windows")] {
            path_delim = ";";
        } else if #[cfg(target_os = "linux")] {
            path_delim = ":";
        } else if #[cfg(target_os = "macos")] {
            path_delim = ":";
        } else {
            panic!("Unsupported platform");
        }
    }

    let path = env::var("PATH")?;
    let elements = path.split(path_delim);
    for element in elements {
        let candidate = PathBuf::from(element).join(exe_name);
        if candidate.is_file() {
            return Ok(candidate);
        }
    }

    Err(anyhow!("Failed to find {}", exe_name))
}
