mod cli;
mod git;

fn main() -> anyhow::Result<()> {
    cli::exec()
}
