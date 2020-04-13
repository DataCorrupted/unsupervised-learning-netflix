use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

use csv::StringRecord;
use itertools::Itertools;
use log::info;
use serde::Deserialize;

use elapsed::measure_time;

trait FromStringRecord {
    fn from_string_record(record: StringRecord) -> Result<Self, Box<dyn Error>>
    where
        Self: std::marker::Sized;
}

fn load_csv<T>(p: PathBuf) -> Result<Vec<T>, Box<dyn Error>>
where
    T: FromStringRecord + std::fmt::Debug,
{
    info!("Loading csv from {:?}", p);
    let (elapsed, ret) = measure_time(|| {
        let file = File::open(p).unwrap();
        let mut rdr = csv::Reader::from_reader(file);
        let mut ret = vec![];
        for result in rdr.records() {
            ret.push(T::from_string_record(result?)?);
        }
        Ok(ret)
    });
    info!("Elapsed {}", elapsed);
    ret
}

pub type Rating = u8;

#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub movie_id: u64,
    pub customer_id: u64,
    pub rating: Rating,
    pub date: String,
}

impl FromStringRecord for Transaction {
    fn from_string_record(record: StringRecord) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            movie_id: record.get(0).unwrap().parse()?,
            customer_id: record.get(1).unwrap().parse()?,
            rating: record.get(2).unwrap().parse().unwrap_or(0),
            date: record.get(3).unwrap().to_string(),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct Movie {
    movie_id: u64,
    year_produced: u16,
    title: String,
}
impl FromStringRecord for Movie {
    fn from_string_record(record: StringRecord) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            movie_id: record.get(0).unwrap().parse()?,
            year_produced: record.get(1).unwrap().parse().unwrap_or(0),
            title: record.get(2).unwrap().to_string(),
        })
    }
}

#[derive(Debug)]
pub struct MetaData {
    num_customers: usize,
    num_movies: usize,
}
impl MetaData {
    pub fn from_data(data: &Data) -> Self {
        Self {
            num_customers: data
                .transactions
                .iter()
                .map(|transaction| transaction.customer_id)
                .unique()
                .collect::<Vec<u64>>()
                .len(),
            num_movies: data.movies.len(),
        }
    }
}

#[allow(dead_code)]
pub struct Data {
    pub transactions: Vec<Transaction>,
    pub movies: Vec<Movie>,
    pub test_data: Vec<Transaction>,
}

impl Data {
    pub fn new<P>(path: P) -> Result<Self, Box<dyn Error>>
    where
        P: Into<PathBuf> + Clone + std::fmt::Debug,
    {
        info!("Loading data from: {:?}", path);
        Ok(Data {
            transactions: load_csv(path.clone().into().join("train.csv"))?,
            movies: load_csv(path.clone().into().join("movie_titles.csv"))?,
            test_data: load_csv(path.clone().into().join("test.csv"))?,
        })
    }
}
