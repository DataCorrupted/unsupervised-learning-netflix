//! ## Recommend NetFlix.
//!
//! This is a homework for ECS271. I know it's kinda better and easier
//! (mostly easier) to use Python. But I do want to try to use rust
//! to do some learning and see how it works.

/// All the configurations for the binary.
/// The config for the models are put in their code.
mod config;

/// A mod to help deal with csv and all data.
mod data;

/// Matrix completion and spectral clustering are put in here.
mod models;

/// Manages all plot related features.
mod plot;

use log::{error, info, warn};
use std::{env, path::Path, process};

use crate::data::Data;

extern crate pretty_env_logger;

/// All the dirty work goes here.
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

    let (metadata, data) = match Data::new(data_path) {
        Ok(d) => d,
        Err(err) => {
            error!("error running example: {}", err);
            process::exit(1);
        }
    };
    let (num_trans, num_tests) = metadata
        .trans_freq
        .iter()
        .zip(metadata.test_freq.iter())
        .fold((0, 0), |(num_trans, num_tests), (&trans, &tests)| {
            (num_trans + trans, num_tests + tests)
        });
    info!("Retrived metadata");
    info!(
        "Total # customers: {}, # movies: {}",
        metadata.num_customers, metadata.num_movies
    );
    info!(
        "Total # of transactions: {}, # of tests: {}",
        num_trans, num_tests
    );

    plot::plot_data_freq(&metadata).expect("Cannot plot freq histogram.");

    plot::plot_initial_matrix(&data, &metadata).expect("Cannot plot initial matrix.");
    info!("Initial matrix plotted.");
    /*
        for model_holder in inventory::iter::<ModelHolder> {
            let mut model = model_holder.get_model();
            model
                .init(&data)
                .train()
                .predict_all_and_output_to_file(&data.test_data)
        }
    */
}
