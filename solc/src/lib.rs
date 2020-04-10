// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

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
        command.arg("/c").arg("solcjs.cmd");
        command
    }
}

use std::path::Path;
use std::{fs, io};

/// Compiles all solidity files in given directory.
pub fn compile<T: AsRef<Path>>(path: T) {
    let mut command = platform::solc();
    command
        // Output contract binary
		.arg("--bin")
        // Output contract abi
		.arg("--abi");
        // Overwrite existing output files (*.abi, *.bin, etc.)
		//.arg("--overwrite")
        // Compile optimized evm-bytecode
        //.arg("--optimize");

    for file in sol_files(&path).expect("Contracts directory is not readable.") {
        command.arg(file);
    }

    let child = command
        .current_dir(path)
        .status()
        .unwrap_or_else(|e| panic!("Error compiling solidity contracts: {}", e));
    assert!(
        child.success(),
        "There was an error while compiling contracts code."
    );
}

fn sol_files<T: AsRef<Path>>(path: T) -> io::Result<Vec<String>> {
    let mut sol_files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let filename = path.file_name()
            .and_then(|os_str| os_str.to_str().to_owned());
        match filename {
            Some(file) if file.ends_with(".sol") => {
                sol_files.push(file.into());
            }
            _ => {}
        }
    }

    Ok(sol_files)
}
