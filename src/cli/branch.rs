const NAME: &str = "branch";
const ARG_GROUP_ID: &str = "branch-group-id";
// Arguments
const BRANCH_NAME_1: &str = "branch_name_1";
const BRANCH_NAME_2: &str = "branch_name_2";
const DELETE: &str = "delete";
const MOVE: &str = "move";

pub fn command() -> Box<dyn super::ExecCommand> {
    struct C;
    impl super::ExecCommand for C {
        fn name(&self) -> &str {
            NAME
        }

        fn command(&self) -> clap::Command {
            clap::Command::new(NAME)
                .about("List, create, delete branches")
                .args(vec![
                    clap::Arg::new(BRANCH_NAME_1)
                        .required(true)
                        .help("Branch name"),
                    clap::Arg::new(DELETE)
                        .short('d')
                        .action(clap::ArgAction::SetTrue)
                        .help("Delete branch"),
                    clap::Arg::new(MOVE)
                        .short('m')
                        .action(clap::ArgAction::SetTrue)
                        .help("Rename branch"),
                ])
                .group(
                    clap::ArgGroup::new(ARG_GROUP_ID)
                        .args([DELETE, MOVE])
                        .required(true),
                )
        }

        fn callback(&self) -> super::CommandFn {
            Box::new(|matches: &clap::ArgMatches| -> anyhow::Result<()> {
                let arg = matches
                    .get_one::<clap::Id>(ARG_GROUP_ID)
                    .unwrap_or(&clap::Id::default())
                    .as_str();

                let branch_name1 = matches
                    .get_one::<String>(BRANCH_NAME_1)
                    .expect("Expected required argument");

                match arg {
                    "move" => 
                }

                Ok(())
            })
        }
    }

    Box::new(C {})
}
