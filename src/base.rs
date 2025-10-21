use clap::Command;
use crate::ctl::environment::{
    init_create_environment_cmd,
    init_current_environment_cmd,
    init_list_environment_cmd,
    init_remove_environment_cmd,
    init_switch_environment_cmd,
};

pub struct SubCmds {
    pub _create: Command,
    pub _list: Command,
    pub _switch: Command,
    pub _current: Command,
    pub _remove: Command,
}

pub struct BaseCmd;
impl BaseCmd {
    pub fn execute(&self) -> Command {
        let mut vem_cmd = Command::new("vem");
        vem_cmd = vem_cmd.version(env!("CARGO_PKG_VERSION"));
        vem_cmd = vem_cmd.about("VEM (Vim Environment Manager) - Manage multiple Vim environments");
        vem_cmd = vem_cmd.subcommand_required(true);
        vem_cmd = vem_cmd.arg_required_else_help(true);

        vem_cmd = init_subcmds(vem_cmd);

        vem_cmd
    }
}

// Config used to apply common settings to subcommands
pub struct CmdCnf {
    pub about: &'static str,
    pub subcommand_help_heading: &'static str,
    pub subcommand_value_name: &'static str,
    pub subcommand_required: bool,
    pub arg_required_else_help: bool,
}

fn set_cmdcnf(mut cmd: Command, cnf: &CmdCnf) -> Command {
    cmd = cmd.about(cnf.about);
    cmd = cmd.subcommand_required(cnf.subcommand_required);
    cmd = cmd.arg_required_else_help(cnf.arg_required_else_help);
    cmd = cmd.subcommand_help_heading(cnf.subcommand_help_heading);
    cmd = cmd.subcommand_value_name(cnf.subcommand_value_name);
    cmd
}

fn init_create_cmd() -> Command {
    let mut create_cmd = Command::new("create");
    let create_cmdcnf = CmdCnf {
        about: "Create a new Vim environment",
        subcommand_help_heading: "RESOURCE",
        subcommand_value_name: "RESOURCE",
        subcommand_required: true,
        arg_required_else_help: true,
    };
    create_cmd = set_cmdcnf(create_cmd, &create_cmdcnf);

    let create_environment_cmd = init_create_environment_cmd();
    create_cmd = create_cmd.subcommand(create_environment_cmd);
    create_cmd
}

fn init_list_cmd() -> Command {
    let mut list_cmd = Command::new("list");
    let list_cmdcnf = CmdCnf {
        about: "List all available environments",
        subcommand_help_heading: "RESOURCE",
        subcommand_value_name: "RESOURCE",
        subcommand_required: true,
        arg_required_else_help: true,
    };
    list_cmd = set_cmdcnf(list_cmd, &list_cmdcnf);

    let list_environment_cmd = init_list_environment_cmd();
    list_cmd = list_cmd.subcommand(list_environment_cmd);
    list_cmd
}

fn init_switch_cmd() -> Command {
    let mut switch_cmd = Command::new("switch");
    let switch_cmdcnf = CmdCnf {
        about: "Switch to a specific environment",
        subcommand_help_heading: "RESOURCE",
        subcommand_value_name: "RESOURCE",
        subcommand_required: true,
        arg_required_else_help: true,
    };
    switch_cmd = set_cmdcnf(switch_cmd, &switch_cmdcnf);

    let switch_environment_cmd = init_switch_environment_cmd();
    switch_cmd = switch_cmd.subcommand(switch_environment_cmd);
    switch_cmd
}

fn init_current_cmd() -> Command {
    let mut current_cmd = Command::new("current");
    let current_cmdcnf = CmdCnf {
        about: "Show the currently active environment",
        subcommand_help_heading: "RESOURCE",
        subcommand_value_name: "RESOURCE",
        subcommand_required: true,
        arg_required_else_help: true,
    };
    current_cmd = set_cmdcnf(current_cmd, &current_cmdcnf);

    let current_environment_cmd = init_current_environment_cmd();
    current_cmd = current_cmd.subcommand(current_environment_cmd);
    current_cmd
}

fn init_remove_cmd() -> Command {
    let mut remove_cmd = Command::new("remove");
    let remove_cmdcnf = CmdCnf {
        about: "Remove an environment",
        subcommand_help_heading: "RESOURCE",
        subcommand_value_name: "RESOURCE",
        subcommand_required: true,
        arg_required_else_help: true,
    };
    remove_cmd = set_cmdcnf(remove_cmd, &remove_cmdcnf);

    let remove_environment_cmd = init_remove_environment_cmd();
    remove_cmd = remove_cmd.subcommand(remove_environment_cmd);
    remove_cmd
}

fn init_subcmds(mut cmd: Command) -> Command {
    let subcmds = SubCmds {
        _create: init_create_cmd(),
        _list: init_list_cmd(),
        _switch: init_switch_cmd(),
        _current: init_current_cmd(),
        _remove: init_remove_cmd(),
    };
    
    cmd = cmd.subcommand(subcmds._create);
    cmd = cmd.subcommand(subcmds._list);
    cmd = cmd.subcommand(subcmds._switch);
    cmd = cmd.subcommand(subcmds._current);
    cmd = cmd.subcommand(subcmds._remove);
    cmd
}