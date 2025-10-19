use std::{env, fs::File, io::stdin};

use crate::config::{Config, TarMode};

mod config;

fn list_contents(cfg: &Config) {
    let archive = if cfg.input_src == config::InputSource::File {
        let mut file = File::open(cfg.input_file.as_str()).unwrap_or_else(|err| {
            panic!("failed to open file {:?}", err);
        });

        cc_tar_rs::read_archive(&mut file)
    } else {
        cc_tar_rs::read_archive(&mut stdin())
    };

    for archived_file in archive.list() {
        println!(
            "{}{}",
            archived_file.file_name_prefix, archived_file.file_name
        );
    }
}

fn run_tar(config: &Config) {
    match config.mode {
        TarMode::List => list_contents(config),
        TarMode::Create => {
            println!("Not supported yet!")
        }
    }
}

fn main() {
    // Initialize logging
    env_logger::init();

    let mut cfg = Config::default();
    let args: Vec<String> = env::args().collect();
    let mut arg_counter: usize = 0;
    while arg_counter < args.len() {
        let arg_value = args[arg_counter].as_str();
        match arg_value {
            "-t" => {
                cfg.mode = TarMode::List;
            }
            "-f" => {
                cfg.input_src = config::InputSource::File;
                arg_counter += 1;
                cfg.input_file = args[arg_counter].as_str().to_string();
            }
            _ => {}
        }
        arg_counter += 1;
    }
    run_tar(&cfg);
}
