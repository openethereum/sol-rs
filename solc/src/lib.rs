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

/// Compiles all solidity files in given directory.
pub fn compile(path: &str) {
    // Find all solidity files
    let sol_files = || -> io::Result<Vec<String>> {
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
    };

    let mut command = platform::solc();
    command
		.arg("--bin")
		.arg("--abi")
        .arg("--optimize");

    for file in sol_files().expect("Contracts directory is not readable.") {
        command.arg(file);
    }

	let child = command
		.current_dir(path)
		.status()
		.unwrap_or_else(|e| panic!("Error compiling solidity contracts: {}", e));
	assert!(child.success(), "There was an error while compiling contracts code.");
}
