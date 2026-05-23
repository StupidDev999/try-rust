use std::env;

use super::Verbosity::Verbosity;

pub struct AppState {
    verbose: Verbosity,
    colours: bool,
    warn: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            verbose: Verbosity::FILE_LINES,
            colours: true,
            warn: false,
        }
    }
}

impl AppState {
    pub fn updateFromCli(&mut self) -> bool {
        let args: Vec<String> = env::args().collect();
        let mut errorArgs = false;
        for arg in &args {
            if arg == "-h" || arg == "--help" {
                println!("Help prompt!");
                return false;
            } else if arg == "-C" || arg == "--disable-colour" {
                self.colours = false;
            } else if arg == "-w" || arg == "--enable-warn" {
                self.warn = true
            } else if arg == "-v0" {
                self.verbose = Verbosity::ONLYFILES
            } else if arg == "-v1" {
                self.verbose = Verbosity::FILE_LINE_NUM
            } else if arg == "-v2" {
                self.verbose = Verbosity::FILE_LINES
            } else if arg == "-v3" {
                self.verbose = Verbosity::FILE_SURR
            } else {
                errorArgs = true;
            }
        }

        if errorArgs {
            println!("Unknown/Erroneous arguments found, ignored them.");
        }

        return true
    }
}
