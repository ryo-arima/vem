use clap::{
    Arg,
    ArgAction,
    Command,
};

pub fn init_create_environment_cmd() -> Command {
    let mut env = Command::new("env");
    env = env.visible_alias("environment");
    env = env.visible_alias("environments");
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

/// list の配下にぶら下げる environment 用の resource コマンド
pub fn init_list_environment_cmd() -> Command {
    let mut env = Command::new("env");
    env = env.visible_alias("environment");
    env = env.visible_alias("environments");
    env = env.about("List environment resources");

    let mut verbose = Arg::new("verbose");
    verbose = verbose
        .help("Show verbose output")
        .short('v')
        .long("verbose")
        .action(ArgAction::SetTrue);
    env = env.arg(verbose);

    env
}

/// switch の配下にぶら下げる environment 用の resource コマンド
pub fn init_switch_environment_cmd() -> Command {
    let mut env = Command::new("env");
    env = env.visible_alias("environment");
    env = env.visible_alias("environments");
    env = env.about("Switch active environment resource");

    let mut name = Arg::new("name");
    name = name
        .help("Environment name to switch to")
        .required(true)
        .value_name("NAME");
    env = env.arg(name);

    env
}

/// current の配下にぶら下げる environment 用の resource コマンド
pub fn init_current_environment_cmd() -> Command {
    let mut env = Command::new("env");
    env = env.visible_alias("environment");
    env = env.visible_alias("environments");
    env = env.about("Show current environment resource");
    env
}

/// remove の配下にぶら下げる environment 用の resource コマンド
pub fn init_remove_environment_cmd() -> Command {
    let mut env = Command::new("env");
    env = env.visible_alias("environment");
    env = env.visible_alias("environments");
    env = env.about("Remove an environment resource");

    let mut name = Arg::new("name");
    name = name
        .help("Environment name to remove")
        .required(true)
        .value_name("NAME");
    env = env.arg(name);

    let mut force = Arg::new("force");
    force = force
        .help("Force removal without confirmation")
        .short('f')
        .long("force")
        .action(ArgAction::SetTrue);
    env = env.arg(force);

    env
}
