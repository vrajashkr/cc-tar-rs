use std::env;

use config::types::{Config, InputSource, TarMode};
use cctar::{constants, tar::run_tar};

pub mod cctar;
pub mod config;

fn main() {
    let mut cfg = Config{
        mode: TarMode::Create,
        block_size: constants::DEFAULT_BLOCK_SIZE_BYTES,
        input_src: InputSource::Stdin,
        input_file: "".to_string(),
    };
    let args: Vec<String> = env::args().collect();
    let mut arg_counter: usize = 0;
    while arg_counter < args.len() {
        let arg_value = args[arg_counter].as_str();
        match arg_value {
            "-t" => {
                cfg.mode = TarMode::List;
            }
            "-f" => {
                cfg.input_src = InputSource::File;
                arg_counter += 1;
                cfg.input_file = args[arg_counter].as_str().to_string();
            }
            _ => {}
        }
        arg_counter += 1;
    }
    run_tar(&cfg);
}
