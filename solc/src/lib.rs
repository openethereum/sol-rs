#[cfg(not(windows))]
mod platform {
	use std::process::Command;

	pub fn solc() -> Command {
        Command::new("solcjs")
	}
}

#[cfg(windows)]
mod platform {
	use std::process::Command;

	pub fn solc() -> Command {
        let command = Command::new("cmd.exe");
        command
			.arg("/c")
			.arg("solcjs.cmd");
        command
	}
}

use std::{fs, io};
use std::path::Path;

/// Compiles all solidity files in given directory.
pub fn compile<T: AsRef<Path>>(path: T) {
    let mut command = platform::solc();
    command
        // Output contract binary
		.arg("--bin")
        // Output contract abi
		.arg("--abi")
        // Overwrite existing output files (*.abi, *.bin, etc.)
		.arg("--overwrite")
        // Compile optimized evm-bytecode
        .arg("--optimize");

    for file in sol_files(&path).expect("Contracts directory is not readable.") {
        command.arg(file);
    }

	let child = command
		.current_dir(path)
		.status()
		.unwrap_or_else(|e| panic!("Error compiling solidity contracts: {}", e));
	assert!(child.success(), "There was an error while compiling contracts code.");
}

fn sol_files<T: AsRef<Path>>(path: T) -> io::Result<Vec<String>> {
    let mut sol_files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let filename = path.file_name().and_then(|os_str| os_str.to_str().to_owned());
        match filename {
            Some(file) if file.ends_with(".sol") => {
                sol_files.push(file.into());
            },
            _ => {},
        }
    }

    Ok(sol_files)
}
