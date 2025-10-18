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
    pub mod error;
    pub mod logger;
}

use anyhow::Result;
use ctl::base::run_cli;

fn main() -> Result<()> {
    run_cli()
}
