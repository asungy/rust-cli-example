const NAME: &str = "clone";
// Arguments
const REPO_URL: &str = "repo_url";
const BARE: &str = "bare";
const DEPTH: &str = "depth";

pub fn command() -> Box<dyn super::ExecCommand> {
    struct C;

    impl super::ExecCommand for C {
        fn name(&self) -> &str {
            NAME
        }

        fn command(&self) -> clap::Command {
            clap::Command::new(NAME)
                .about("Clone an existing repository.")
                .args(vec![
                    clap::Arg::new(BARE)
                        .long("bare")
                        .action(clap::ArgAction::SetTrue)
                        .help("Clone a bare repository"),
                    clap::Arg::new(DEPTH)
                        .long("depth")
                        .value_parser(clap::value_parser!(usize))
                        .help("Shallow clone with truncated history"),
                    clap::Arg::new(REPO_URL)
                        .required(true)
                        .value_parser(clap::value_parser!(String))
                        .num_args(1)
                        .help("Repository URL"),
                ])
        }

        fn callback(&self) -> super::CommandFn {
            Box::new(|matches: &clap::ArgMatches| -> anyhow::Result<()> {
                let bare: Option<bool> = matches.get_one::<bool>(BARE).copied();
                let depth: Option<usize> = matches.get_one::<usize>(DEPTH).copied();
                let repo: String = matches
                    .get_one::<String>(REPO_URL)
                    .expect("Expected required argument")
                    .to_owned();
                let arg = crate::git::CloneArg { bare, depth };

                crate::git::clone(repo, arg);

                Ok(())
            })
        }
    }

    Box::new(C {})
}
