//! ## Recommend NetFlix.
//!
//! This is a homework for ECS271. I know it's kinda better and easier
//! (mostly easier) to use Python. But I do want to try to use rust
//! to do some learning and see how it works.
//! 
//! It is possible to combine doc with rust just like Java does.  
//! What's more, you can even write math formular with the help of
//! crate [katex-doc](https://crates.io/crates/katex-doc). For example, 
//! I can inline formular 
//! $` f(x) = \int_{-\infty}^\infty \hat f(\xi)\,e^{2 \pi i \xi x} \,d\xi `$
//! or simply do:
//! 
//! ```math
//! f(x) = \int_{-\infty}^\infty
//! \hat f(\xi)\,e^{2 \pi i \xi x}
//! \,d\xi
//! ```
//! 
//! That's the reason I am going to put this homework's report here.

/// All the configurations for the binary.
/// The config for the models are put in their code, NOT here.
mod config;

/// Deals with all data.
mod data;

/// Matrix completion and spectral clustering are put in here.
mod models;

/// Manages all plot related features.
mod plot;

/// Handles input/output of the data.
mod io;

/// Any common algorithms go here.
mod algorithm;

use log::{error, info, warn};
use std::{env, path::Path, process};

use crate::data::Data;
use crate::io::DumpToFile;
use crate::models::ModelHolder;

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

    let data = match Data::new(data_path) {
        Ok(d) => d,
        Err(err) => {
            error!("error running example: {}", err);
            process::exit(1);
        }
    };
    let metadata = &data.metadata;
    let (num_trans, num_tests) = metadata
        .trans_freq
        .iter()
        .zip(metadata.tests_freq.iter())
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

    /*
    plot::plot_data_freq(metadata).expect("Cannot plot freq histogram.");

    plot::plot_initial_matrix(&data).expect("Cannot plot initial matrix.");
    info!("Initial matrix plotted.");
    */
    for model_holder in inventory::iter::<ModelHolder> {
        let mut model = model_holder.get_model();
        model
            .init(&data)
            .train()
            .predict_all(&data.test_data)
            .dump_to_file(format!("{}.txt", model_holder.get_name()));
    }
}
