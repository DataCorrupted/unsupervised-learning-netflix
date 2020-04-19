use crate::data::*;
use csv::StringRecord;
use elapsed::measure_time;
use log::info;
use std::{error::Error, fmt::Display, fs::File, io::Write, ops::Sub, path::PathBuf};

/// Converts a `StringRecord` to our type.
pub trait FromStringRecord {
    fn from_string_record(record: StringRecord) -> Result<Self, Box<dyn Error>>
    where
        Self: std::marker::Sized;
}

impl FromStringRecord for Movie {
    fn from_string_record(record: StringRecord) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            // movie_id starts counting from 1, we don't like that.
            movie_id: record.get(0).unwrap().parse::<usize>()?.sub(1),
            year_produced: record.get(1).unwrap().parse().unwrap_or(0),
            title: record.get(2).unwrap().to_string(),
        })
    }
}

impl FromStringRecord for Transaction {
    fn from_string_record(record: StringRecord) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            // movie_id starts counting from 1, we don't like that.
            movie_id: record.get(0).unwrap().parse::<usize>()?.sub(1),
            customer_id: record.get(1).unwrap().parse()?,
            rating: record.get(2).unwrap().parse().unwrap_or(0),
            date: record.get(3).unwrap().to_string(),
        })
    }
}

/// Convert a CSV file to `Vec<T>` if `T: FromStringRecord`.
pub trait FromCsv: FromStringRecord {
    fn from_csv(p: PathBuf) -> Result<Vec<Self>, Box<dyn Error>>
    where
        Self: std::marker::Sized,
    {
        info!("Loading csv from {:?}", p);
        let (elapsed, ret) = measure_time(|| {
            let file = File::open(p).unwrap();
            let mut rdr = csv::Reader::from_reader(file);
            let mut ret = vec![];
            for result in rdr.records() {
                ret.push(Self::from_string_record(result?)?);
            }
            Ok(ret)
        });
        info!("Elapsed {}", elapsed);
        ret
    }
}
impl FromCsv for Movie {}
impl FromCsv for Transaction {}

/// Dump something into a file.
pub trait DumpToFile {
    /// By default it outputs to `<model-name>.txt` in the working directory.
    fn dump_to_file(&self, file_name: String);
}
impl<T: Display> DumpToFile for Vec<T> {
    fn dump_to_file(&self, file_name: String) {
        let mut file = File::create(file_name.clone())
            .expect(format!("Unable to create file {}", file_name).as_str());
        self.iter().for_each(|t| {
            file.write_fmt(format_args!("{}\n", t))
                .expect(format!("Write to file {} failed.", file_name).as_str());
        });
    }
}
