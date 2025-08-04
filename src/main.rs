mod cli;
mod git;

use std::process::ExitCode;

fn main() -> ExitCode {
    match cli::exec() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::FAILURE
        }
    }
}
