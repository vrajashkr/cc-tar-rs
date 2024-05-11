use std::env;

use config::config::{Config, Mode};
use cctar::{constants, tar::run_tar};

pub mod cctar;
pub mod config;

#[allow(clippy::single_match)]
fn main() {
    let mut cfg = Config{
        mode: Mode::Create,
        block_size: constants::DEFAULT_BLOCK_SIZE_BYTES
    };
    let args: Vec<String> = env::args().collect();
    for arg in args {
        let arg_value = arg.as_str();
        match arg_value {
            "-t" => {
                cfg.mode = Mode::List;
            }
            _ => {}
        }
    }
    run_tar(&cfg);
}
