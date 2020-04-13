pub mod matrix_completion;
pub mod spectral_clustering;

use std::{fs::File, io::Write};

use crate::data::{Rating, Transaction};

trait Models {
    const NAME: &'static str = "PredModel";
    fn predict(&self, trans: &Transaction) -> Rating;
    fn predict_all_and_output_to_file(&self, test_data: &Vec<Transaction>) {
        let file_name = format!("{}.txt", Self::NAME);
        let mut file = File::create(file_name.clone())
            .expect(format!("Unable to create file {}", file_name).as_str());
        test_data.iter().for_each(|t| {
            file.write_fmt(format_args!("{}\n", self.predict(t)))
                .expect(format!("Write to file {} failed.", file_name).as_str());
        });
    }
}
