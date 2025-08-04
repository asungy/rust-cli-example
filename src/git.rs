pub enum BranchArg {
    Delete {
        branch: String,
    },
    Move {
        old_branch: String,
        new_branch: String,
    },
}

pub fn branch(arg: BranchArg) {
    match arg {
        BranchArg::Delete { branch } => println!("Deleting {branch}."),
        BranchArg::Move {
            old_branch,
            new_branch,
        } => println!("Renaming branch from `{old_branch}` to `{new_branch}`."),
    }
}

pub struct CloneArg {
    bare: Option<bool>,
    depth: Option<usize>,
}

pub fn clone(repo: String, arg: CloneArg) {
    if let Some(bare) = arg.bare
        && bare
    {
        println!("Bare flag set.");
    }

    if let Some(depth) = arg.depth {
        println!("Depth argument set to {depth}.");
    }

    println!("Cloning repo: {repo}");
}

pub enum RemoteSubcommand {
    Add { name: String, url: String },
    Remove { name: String },
}

pub fn remote(subcommand: RemoteSubcommand) {
    match subcommand {
        RemoteSubcommand::Add { name, url } => println!("Adding remote {url} as {name}."),
        RemoteSubcommand::Remove { name } => println!("Removing remote {name}."),
    }
}
