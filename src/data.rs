use log::{info, warn};
use rusty_machine::prelude::*;
use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    fmt::Debug,
    path::PathBuf,
};

use crate::config;
use crate::io::FromCsv;

pub type Rating = u8;
/// `Transaction` is a customer's behavior.config
///
/// `Transcction` consists of the `movie_id` he bought,
/// `customer_id` to tell us who he is, his `rating`
/// (between 1 and 5 inclusive), and `date`
///
/// If 'rating' is 0 then this `Transaction` is in test set.
#[derive(Debug, Clone)]
pub struct Transaction {
    pub movie_id: usize,
    pub customer_id: usize,
    pub rating: Rating,
    pub date: String,
}

#[derive(Debug)]
pub struct Movie {
    pub movie_id: usize,
    pub year_produced: u16,
    pub title: String,
}

#[derive(Debug, Clone)]
pub struct MetaData {
    pub num_customers: usize,
    pub num_movies: usize,
    pub num_train: usize,
    pub num_cross_valid: usize,
    pub trans_freq: Vec<u32>,
    pub tests_freq: Vec<u32>,
}

/// `Data` holds all `Transaction`s, `Movie`s and test set,
/// which is also in the form a `Transaction`.
pub struct Data {
    pub metadata: MetaData,
    pub train: Vec<Transaction>,
    pub cross_valid: Vec<Transaction>,
    pub movies: Vec<Movie>,
    pub test_data: Vec<Transaction>,
}

impl Data {
    pub fn new<P>(path: P) -> Result<Self, Box<dyn Error>>
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

        // 20% of training data is used for cross validation.
        let num_cross_valid = transactions.len() / 5;
        let num_train = transactions.len() - num_cross_valid;

        let mut transactions: VecDeque<Transaction> = transactions.into();

        Ok(Data {
            metadata: MetaData {
                num_customers: virtual_id as usize,
                num_movies: movies.len(),
                num_train: num_train,
                num_cross_valid: num_cross_valid,
                trans_freq: trans_freq,
                tests_freq: tests_freq,
            },
            train: transactions.drain(0..num_train).collect(),
            cross_valid: transactions.drain(0..num_cross_valid).collect(),
            movies: movies,
            test_data: test_data,
        })
    }
}

pub trait TrainingDataToMatrix {
    fn training_data_to_matrix(&self) -> Matrix<f64>;
}

impl TrainingDataToMatrix for Data {
    fn training_data_to_matrix(&self) -> Matrix<f64> {
        let mut ret = Matrix::<f64>::zeros(self.metadata.num_customers, self.metadata.num_movies);
        self.train.iter().for_each(|t| {
            ret[[t.customer_id, t.movie_id]] = t.rating as f64;
        });
        ret
    }
}