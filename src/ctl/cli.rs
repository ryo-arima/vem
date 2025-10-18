use crate::ent::{ExitCode, VemError};
use crate::usc::EnvironmentManager;
use anyhow::Result;
use clap::{Parser, Subcommand};

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
    let env_manager = EnvironmentManager::new()?;

    match command {
        Commands::Create {
            name,
            description: _,
        } => {
            env_manager.create_environment(name.as_str())?;
            println!("Environment '{}' created successfully", name);
        }
        Commands::List { verbose } => {
            let environments = env_manager.list_environments()?;
            if environments.is_empty() {
                println!("No environments found");
            } else {
                for env in environments {
                    if verbose {
                        println!("{} ({})", env.name, env.path.display());
                    } else {
                        println!("{}", env.name);
                    }
                }
            }
        }
        Commands::Switch { name } => {
            env_manager.switch_environment(name.as_str())?;
            println!("Switched to environment '{}'", name);
        }
        Commands::Current => match env_manager.get_current_environment() {
            Ok(env) => println!("{}", env.name),
            Err(_) => println!("No environment currently active"),
        },
        Commands::Remove { name, force } => {
            if !force {
                print!(
                    "Are you sure you want to remove environment '{}'? (y/N): ",
                    name
                );
                use std::io::{self, Write};
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let first_non_ws = input.chars().find(|c| !c.is_whitespace());
                let yes = matches!(first_non_ws, Some('y') | Some('Y'));
                if !yes {
                    println!("Operation cancelled");
                    return Ok(());
                }
            }

            env_manager.remove_environment(name.as_str())?;
            println!("Environment '{}' removed successfully", name);
        }
    }

    Ok(())
}

pub fn run_cli() -> Result<()> {
    let cli = Cli::parse();

    // Set up logging/output level based on flags
    if cli.verbose {
        unsafe { std::env::set_var("RUST_LOG", "debug"); }
    } else if cli.quiet {
        unsafe { std::env::set_var("RUST_LOG", "error"); }
    }

    // Handle the command
    match handle_command(cli.command) {
        Ok(_) => std::process::exit(ExitCode::Success as i32),
        Err(err) => {
            if !cli.quiet {
                eprintln!("Error: {}", err);
            }
            let exit_code = match err.downcast_ref::<VemError>() {
                Some(vem_err) => ExitCode::from(vem_err) as i32,
                None => ExitCode::GeneralError as i32,
            };
            std::process::exit(exit_code);
        }
    }
}