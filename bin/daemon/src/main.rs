#[macro_use] extern crate clap;
#[macro_use] extern crate log;

extern crate crypto;

extern crate bin_common as common;

use clap::App;
use common::config::Config;


fn load_config() -> Config {
    let cli_yaml = load_yaml!("../resources/daemon.yml");
    let matches = App::from_yaml(cli_yaml).get_matches();

    Config {
        log_level: matches.value_of("log-level").unwrap().parse::<u8>().unwrap(),
        log_file: String::from(matches.value_of("log-file").unwrap())
    }
}

fn main() {
    // Command Line Arguments
    let config = load_config();

    // Logging
    common::logger::init(config).expect("Failed to initialise logger");

    // Main
    run();
}

fn run() {
    info!("{}", format_args!("Coin Name {} - {}", cryptonote_config::VERSION, cryptonote_config::RELEASE_NAME));

    // cryptonote_core::init();
}
