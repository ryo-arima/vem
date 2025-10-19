#![allow(non_camel_case_types)]

// Module declarations
mod cnf {
    pub mod application;
}

mod ent {
    pub mod model {
        pub mod environment;
    }
    pub mod request {
        pub mod environment;
    }
    pub mod response {
        pub mod environment;
    }
}

mod rep {
    pub mod environment;
}

mod usc {
    pub mod environment;
}

mod ctl {
    pub mod base;
    pub mod environment;
}

mod util {
    pub mod mcode;
    pub mod error;
    pub mod logger;
}

use anyhow::Result;
use ctl::base::run_cli;
use util::mcode::{
    format_message,
    log_level_t::{DEBUG, ERROR},
    VSS1, VSE1, VSE2,
};

fn main() -> Result<()> {
    // Application start
    if std::env::var("RUST_LOG").is_ok() {
        eprintln!("{}", format_message(DEBUG, VSS1, ""));
    }

    let result = run_cli();

    // Application exit
    match &result {
        Ok(_) => {
            if std::env::var("RUST_LOG").is_ok() {
                eprintln!("{}", format_message(DEBUG, VSE1, ""));
            }
        },
        Err(e) => {
            eprintln!("{}", format_message(ERROR, VSE2, &e.to_string()));
        },
    }

    result
}
