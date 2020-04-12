mod data;

use std::process;
//use std::time::Duration;
//use stopwatch::Stopwatch;
use log::info;

use crate::data::{Data, MetaData};

extern crate pretty_env_logger;
/*
static mut STOP_WATCH: Option<Stopwatch> = None;

fn tic() {
    unsafe {
        STOP_WATCH = Some(Stopwatch::start_new());
    }
}

fn toc() -> Duration {
    unsafe {
        match STOP_WATCH {
            Some(s) => s.elapsed(),
            None => Duration::from_secs(0),
        }
    }
}
*/
fn main() {
    pretty_env_logger::init();

    let data = match Data::new() {
        Ok(d) => d,
        Err(err) => { 
            println!("error running example: {}", err);
            process::exit(1);
        }
    };
    info!("Retrived: {:?}", MetaData::from_data(&data));
}
