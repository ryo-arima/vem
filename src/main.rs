include!("mod.rs");
mod base;
use crate::base::BaseCmd;

fn main() {
    let base_cmd = BaseCmd;
    // Build CLI and parse (discard result for now to avoid unused warnings)
    let _matches = base_cmd.execute().get_matches();
}
