mod branch;
mod clone;
mod remote;

const NAME: &str = std::env!("CARGO_PKG_NAME");
const VERSION: &str = std::env!("CARGO_PKG_VERSION");

type CommandFn = Box<dyn FnOnce(&clap::ArgMatches) -> anyhow::Result<()>>;
trait ExecCommand {
    fn name(&self) -> &str;
    fn command(&self) -> clap::Command;
    fn callback(&self) -> CommandFn;
}

fn exec_commands() -> Vec<Box<dyn ExecCommand>> {
    vec![]
}

fn subcommands() -> Vec<clap::Command> {
    exec_commands().iter().map(|c| c.command()).collect()
}

fn callback(name: &str) -> Option<CommandFn> {
    exec_commands()
        .iter()
        .find(|c| c.name() == name)
        .map(|c| c.callback())
}

fn cli() -> clap::Command {
    clap::Command::new(NAME)
        .version(VERSION)
        .subcommand_required(true)
        .subcommands(subcommands())
}

pub fn exec() -> anyhow::Result<()> {
    let matches = cli().get_matches();

    if let Some((name, arg_matches)) = matches.subcommand() {
        callback(name).expect("This should not happen. Something is not implemented properly.")(
            arg_matches,
        )
    } else {
        cli().print_long_help().map_err(|e| anyhow::Error::from(e))
    }
}
