mod config;
mod data;

use log::{error, info, warn};
use std::{env, path::Path, process};

use crate::data::{Data, MetaData};

extern crate pretty_env_logger;

fn main() {
    // By default we show all logs.
    if env::var(config::RUST_LOG).is_err() {
        env::set_var(config::RUST_LOG, "trace");
    }
    // Init logger.
    pretty_env_logger::init();

    let data_path = match env::var(config::DATA_PATH) {
        Ok(val) => Path::new(&val).to_path_buf(),
        Err(_) => {
            warn!(
                "${} not set, using ${}/data/ as default.",
                config::DATA_PATH,
                config::RECOMMEND_HOME
            );
            match env::var(config::RECOMMEND_HOME) {
                Ok(val) => Path::new(&val).join("data"),
                Err(_) => {
                    error!("${} not set.", config::RECOMMEND_HOME);
                    process::exit(1);
                }
            }
        }
    };

    let data = match Data::new(data_path) {
        Ok(d) => d,
        Err(err) => {
            println!("error running example: {}", err);
            process::exit(1);
        }
    };
    info!("Retrived: {:?}", MetaData::from_data(&data));
}
