const NAME: &str = "branch";
const ARG_GROUP_ID: &str = "branch-group-id";
// Arguments
const BRANCH_NAME: &str = "branch_name";
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
                    clap::Arg::new(BRANCH_NAME)
                        .required(true)
                        .num_args(0..)
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
                    .expect("Expected required argument")
                    .as_str();

                let branch_name: Vec<String> = matches
                    .get_many::<String>(BRANCH_NAME)
                    .unwrap_or_default()
                    .cloned()
                    .collect();

                match arg {
                    MOVE => {
                        crate::git::branch(crate::git::BranchArg::Move {
                            old_branch: branch_name
                                .get(0)
                                .ok_or(anyhow::anyhow!("Expected old branch name"))?
                                .to_owned(),
                            new_branch: branch_name
                                .get(1)
                                .ok_or(anyhow::anyhow!("Expected new branch name"))?
                                .to_owned(),
                        });
                    }
                    DELETE => crate::git::branch(crate::git::BranchArg::Delete {
                        branch: branch_name
                            .get(0)
                            .ok_or(anyhow::anyhow!("Expected branch name"))?
                            .to_owned(),
                    }),
                    _ => unreachable!("Unexpected branch argument."),
                }

                Ok(())
            })
        }
    }

    Box::new(C {})
}
