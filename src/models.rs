pub mod matrix_completion;
pub mod spectral_clustering;

use std::{fmt::Debug, fs::File, io::Write};

use crate::data::{Data, Rating, Transaction};

pub trait DefaultModel: Debug {
    fn default(&self) -> Box<dyn Model>;
}
impl<T> DefaultModel for T
where
    T: Model + Default + Debug + 'static,
{
    fn default(&self) -> Box<dyn Model> {
        Box::new(T::default())
    }
}
pub struct ModelHolder {
    inner: Box<dyn DefaultModel>,
}
impl ModelHolder {
    pub fn new(d_model: Box<dyn DefaultModel>) -> Self {
        Self { inner: d_model }
    }
    pub fn get_model(&self) -> Box<dyn Model> {
        self.inner.default()
    }
}

pub trait Model {
    fn get_name(&self) -> &'static str {
        "GenericModel"
    }
    fn init(&mut self, data: &Data) -> &mut dyn Model;
    fn train(&mut self) -> &mut dyn Model;
    fn predict(&self, trans: &Transaction) -> Rating;
    fn predict_all_and_output_to_file(&self, test_data: &Vec<Transaction>) {
        let file_name = format!("{}.txt", self.get_name());
        let mut file = File::create(file_name.clone())
            .expect(format!("Unable to create file {}", file_name).as_str());
        test_data.iter().for_each(|t| {
            file.write_fmt(format_args!("{}\n", self.predict(t)))
                .expect(format!("Write to file {} failed.", file_name).as_str());
        });
    }
}

inventory::collect!(ModelHolder);
