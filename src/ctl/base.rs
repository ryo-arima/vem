use crate::usc::environment::environment_manager_t;
use crate::util::mcode::{
    format_message,
    log_level_t::{
        INFO,
        NOTICE,
        WARN
    },
    VCC_CONFIRM, VCC_CANCEL,
    VEC1,
    VEL1,
    VEL3,
    VES1,
    VECU1,
    VECU2,
    VEU1, 
    VED1,
};
use anyhow::Result;
use clap::{
    Parser,
    Subcommand
};

/// Available VEM commands
#[derive(Subcommand)]
pub enum Commands {
    /// Create a new Vim environment
    Create {
        /// Name of the environment to create
        name: String,
        /// Description for the environment
        #[arg(short, long)]
        description: Option<String>,
    },
    /// List all available environments
    List {
        /// Show detailed information
        #[arg(short, long)]
        verbose: bool,
    },
    /// Switch to a different environment
    Switch {
        /// Name of the environment to switch to
        name: String,
    },
    /// Show the current active environment
    Current,
    /// Update an environment's metadata
    Update {
        /// Name of the environment to update
        name: String,
        /// New description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Remove an environment
    Remove {
        /// Name of the environment to remove
        name: String,
        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
    },
}

/// VEM (Vim Environment Manager) - Manage multiple Vim environments
#[derive(Parser)]
#[command(name = "vem")]
#[command(version = "0.1.0")]
#[command(about = "VEM (Vim Environment Manager) - A tool to manage multiple Vim configurations")]
#[command(
    long_about = "VEM is a command-line tool written in Rust for efficiently managing multiple Vim environments. Switch between different .vim configurations easily based on your needs and preferences."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Suppress non-essential output
    #[arg(short, long, global = true)]
    pub quiet: bool,
}

/// Handle the execution of a VEM command
pub fn handle_command(command: Commands) -> Result<()> {
    let env_manager = environment_manager_t::new()?;

    match command {
        Commands::Create { name, description } => {
            env_manager.create_environment(&name, description)?;
            println!("{}", format_message(INFO, VEC1, &format!("Environment '{}'", name)));
        }
        Commands::List { verbose } => {
            let environments = env_manager.list_environments()?;
            if environments.is_empty() {
                println!("{}", format_message(NOTICE, VEL3, ""));
            } else {
                println!("{}", format_message(INFO, VEL1, ""));
                for env in environments {
                    if verbose {
                        println!("{} ({})", env.name(), env.path().display());
                    } else {
                        println!("{}", env.name());
                    }
                }
            }
        }
        Commands::Switch { name } => {
            env_manager.switch_environment(&name)?;
            println!("{}", format_message(INFO, VES1, &format!("Environment '{}'", name)));
        }
        Commands::Current => match env_manager.get_current_environment() {
            Ok(env) => {
                println!(
                    "{}",
                    format_message(INFO, VECU1, &format!("Current: {}", env.name()))
                );
            }
            Err(_) => {
                println!("{}", format_message(NOTICE, VECU2, ""));
            }
        },
        Commands::Update { name, description } => {
            env_manager.update_environment(&name, description)?;
            println!("{}", format_message(INFO, VEU1, &format!("Environment '{}'", name)));
        }
        Commands::Remove { name, force } => {
            if !force {
                print!(
                    "{} ",
                    format_message(WARN, VCC_CONFIRM, &format!("Remove environment '{}'? (y/N):", name))
                );
                use std::io::{self, Write};
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let first_non_ws = input.chars().find(|c| !c.is_whitespace());
                let yes = matches!(first_non_ws, Some('y') | Some('Y'));
                if !yes {
                    println!("{}", format_message(NOTICE, VCC_CANCEL, ""));
                    return Ok(());
                }
            }

            env_manager.remove_environment(&name)?;
            println!("{}", format_message(INFO, VED1, &format!("Environment '{}'", name)));
        }
    }

    Ok(())
}

pub fn run_cli() -> Result<()> {
    let cli = Cli::parse();

    // Set up logging/output level based on flags
    if cli.verbose {
        unsafe {
            std::env::set_var("RUST_LOG", "debug");
        }
    } else if cli.quiet {
        unsafe {
            std::env::set_var("RUST_LOG", "error");
        }
    }

    // Handle the command
    handle_command(cli.command)
}
