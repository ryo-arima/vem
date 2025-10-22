use clap::{
    Arg,
    ArgAction,
    Command,
};

pub fn init_create_environment_cmd() -> Command {
    let mut env = Command::new("env");
    env = env.visible_alias("environment");
    env = env.about("Create an environment resource");

    let mut name = Arg::new("name");
    name = name
        .help("Environment name")
        .required(true)
        .value_name("NAME");
    env = env.arg(name);

    let mut description = Arg::new("description");
    description = description
        .help("Optional description")
        .short('d')
        .long("description")
        .value_name("TEXT");
    env = env.arg(description);

    env
}

pub fn init_list_environment_cmd() -> Command {
    let mut envs = Command::new("envs");
    envs = envs.visible_alias("environments");
    envs = envs.about("List environment resources");

    let mut verbose = Arg::new("verbose");
    verbose = verbose
        .help("Show verbose output")
        .short('v')
        .long("verbose")
        .action(ArgAction::SetTrue);
    envs = envs.arg(verbose);

    envs
}

pub fn init_switch_environment_cmd() -> Command {
    let mut env = Command::new("env");
    env = env.visible_alias("environment");
    env = env.about("Switch active environment resource");

    let mut name = Arg::new("name");
    name = name
        .help("Environment name to switch to")
        .required(true)
        .value_name("NAME");
    env = env.arg(name);

    env
}

pub fn init_current_environment_cmd() -> Command {
    let mut env = Command::new("env");
    env = env.visible_alias("environment");
    env = env.about("Show current environment resource");
    env
}

pub fn init_remove_environment_cmd() -> Command {
    let mut env = Command::new("env");
    env = env.visible_alias("environment");
    env = env.about("Remove an environment resource");

    let mut name = Arg::new("name");
    name = name
        .help("Environment name to remove")
        .required(true)
        .value_name("NAME");
    env = env.arg(name);

    env
}
