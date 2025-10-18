mod cnf;
mod ctl;
mod ent;
mod rep;
mod usc;

use anyhow::Result;
use ctl::cli::run_cli;

fn main() -> Result<()> {
    run_cli()
}
