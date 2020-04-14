use std::{collections::HashMap, error::Error, fmt::Debug, path::PathBuf};

use crate::config;
use crate::io::FromCsv;

use log::{info, warn};

pub type Rating = u8;
/// `Transaction` is a customer's behavior.config
///
/// `Transcction` consists of the `movie_id` he bought,
/// `customer_id` to tell us who he is, his `rating`
/// (between 1 and 5 inclusive), and `date`
///
/// If 'rating' is 0 then this `Transaction` is in test set.
#[derive(Debug)]
pub struct Transaction {
    pub movie_id: u64,
    pub customer_id: u64,
    pub rating: Rating,
    pub date: String,
}

#[derive(Debug)]
pub struct Movie {
    pub movie_id: u64,
    pub year_produced: u16,
    pub title: String,
}

#[derive(Debug, Clone)]
pub struct MetaData {
    pub num_customers: u32,
    pub num_movies: u32,
    pub trans_freq: Vec<u32>,
    pub tests_freq: Vec<u32>,
}

/// `Data` holds all `Transaction`s, `Movie`s and test set,
/// which is also in the form a `Transaction`.
pub struct Data {
    pub transactions: Vec<Transaction>,
    pub movies: Vec<Movie>,
    pub test_data: Vec<Transaction>,
}

impl Data {
    pub fn new<P>(path: P) -> Result<(MetaData, Self), Box<dyn Error>>
    where
        P: Into<PathBuf> + Clone + Debug,
    {
        info!("Loading data from: {:?}", path);
        let mut transactions =
            Transaction::from_csv(path.clone().into().join(config::TRAINING_DATA))?;
        let movies = Movie::from_csv(path.clone().into().join("movie_titles.csv"))?;
        let mut test_data = Transaction::from_csv(path.clone().into().join("test.csv"))?;
        let mut virtual_id_map = HashMap::new();
        let mut virtual_id = 0;
        let mut trans_freq = Vec::new();
        transactions.iter_mut().for_each(|t| {
            let idx = *virtual_id_map.entry(t.customer_id).or_insert_with(|| {
                virtual_id += 1;
                trans_freq.push(0);
                virtual_id - 1
            });
            t.customer_id = idx;
            trans_freq[idx as usize] += 1;
        });
        let mut tests_freq = vec![0; virtual_id as usize];
        test_data.iter_mut().for_each(|t| {
            let idx = *virtual_id_map.entry(t.customer_id).or_insert_with(|| {
                warn!(
                    "How come a customer(id: {}) is in testing set but not in training set? \
                      Setting its virtial id to {}.",
                    t.customer_id, virtual_id
                );
                virtual_id += 1;
                trans_freq.push(0);
                tests_freq.push(0);
                virtual_id - 1
            });
            t.customer_id = idx;
            tests_freq[idx as usize] += 1;
        });
        Ok((
            MetaData {
                num_customers: virtual_id as u32,
                num_movies: movies.len() as u32,
                trans_freq: trans_freq,
                tests_freq: tests_freq,
            },
            Data {
                transactions: transactions,
                movies: movies,
                test_data: test_data,
            },
        ))
    }
}
