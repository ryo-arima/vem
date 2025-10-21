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

        // build and add subcmds
        let SubCmds {
            _create,
            _list,
            _switch,
            _current,
            _remove
        }  = init_subcmds();
        vem_cmd = vem_cmd.subcommand(_create);
        vem_cmd = vem_cmd.subcommand(_list);
        vem_cmd = vem_cmd.subcommand(_switch);
        vem_cmd = vem_cmd.subcommand(_current);
        vem_cmd = vem_cmd.subcommand(_remove);

        vem_cmd
    }
}

// init_* functions for each subcmd
fn init_create_cmd() -> Command {
    let mut cmd = Command::new("create");
    cmd = cmd.about("Create a new Vim environment");
    cmd = cmd.subcommand_required(true);
    cmd = cmd.arg_required_else_help(true);
    cmd = cmd.subcommand_help_heading("RESOURCE");
    cmd = cmd.subcommand_value_name("RESOURCE");

    // attach resource subcommand from ctl
    let create_environment_cmd = init_create_environment_cmd();
    cmd = cmd.subcommand(create_environment_cmd);
    cmd
}

fn init_list_cmd() -> Command {
    let mut cmd = Command::new("list");
    cmd = cmd.about("List all available environments");
    cmd = cmd.subcommand_required(true);
    cmd = cmd.arg_required_else_help(true);
    cmd = cmd.subcommand_help_heading("RESOURCE");
    cmd = cmd.subcommand_value_name("RESOURCE");

    // attach resource subcommand from ctl
    let list_environment_cmd = init_list_environment_cmd();
    cmd = cmd.subcommand(list_environment_cmd);
    cmd
}

fn init_switch_cmd() -> Command {
    let mut cmd = Command::new("switch");
    cmd = cmd.about("Switch to a specific environment");
    cmd = cmd.subcommand_required(true);
    cmd = cmd.arg_required_else_help(true);
    cmd = cmd.subcommand_help_heading("RESOURCE");
    cmd = cmd.subcommand_value_name("RESOURCE");

    // attach resource subcommand from ctl
    let switch_environment_cmd = init_switch_environment_cmd();
    cmd = cmd.subcommand(switch_environment_cmd);
    cmd
}

fn init_current_cmd() -> Command {
    let mut cmd = Command::new("current");
    cmd = cmd.about("Show the currently active environment");
    cmd = cmd.subcommand_required(true);
    cmd = cmd.arg_required_else_help(true);
    cmd = cmd.subcommand_help_heading("RESOURCE");
    cmd = cmd.subcommand_value_name("RESOURCE");

    // attach resource subcommand from ctl
    let current_environment_cmd = init_current_environment_cmd();
    cmd = cmd.subcommand(current_environment_cmd);
    cmd
}

fn init_remove_cmd() -> Command {
    let mut cmd = Command::new("remove");
    cmd = cmd.about("Remove an environment");
    cmd = cmd.subcommand_required(true);
    cmd = cmd.arg_required_else_help(true);
    cmd = cmd.subcommand_help_heading("RESOURCE");
    cmd = cmd.subcommand_value_name("RESOURCE");

    // attach resource subcommand from ctl
    let remove_environment_cmd = init_remove_environment_cmd();
    cmd = cmd.subcommand(remove_environment_cmd);
    cmd
}

fn init_subcmds() -> SubCmds {
    SubCmds {
        _create: init_create_cmd(),
        _list: init_list_cmd(),
        _switch: init_switch_cmd(),
        _current: init_current_cmd(),
        _remove: init_remove_cmd(),
    }
}