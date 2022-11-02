use anyhow::{anyhow, Result};
use cfg_if::cfg_if;
use std::{env, fs, path::PathBuf, process::Command};

fn main() -> Result<()> {
    // We need the full path to the python executable for Command::new()
    let python_exe = find_python_exe()?;

    let mut args: Vec<String> = env::args().collect();
    let exe = fs::canonicalize(&args[0])?;
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
    let path_delim;
    cfg_if! {
        if #[cfg(target_os = "windows")] {
            exe_name = "python.exe";
            path_delim = ";";
        } else if #[cfg(target_os = "linux")] {
            exe_name = "python";
            path_delim = ":";
        } else if #[cfg(target_os = "macos")] {
            exe_name = "python3";
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

    // On Windows perhaps we could look in the registry under:
    // HKEY_LOCAL_MACHINE\Software\Python\PythonCore\<version>\InstallPath

    Err(anyhow!("Failed to find {}", exe_name))
}
