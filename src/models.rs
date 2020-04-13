/// Matrix completion.
pub mod matrix_completion;
/// Spectral clustering.
pub mod spectral_clustering;

use std::{fmt::Debug, fs::File, io::Write};

use crate::data::{Data, Rating, Transaction};

/// `DefaultModel` generate uninitialized `Model`.
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

/// `ModelHolder` is a helper struct that holds all
/// uninitialized models for further initialization.
/// All models are required to be `Model`, `Default`,
/// and `Debug`
#[allow(dead_code)]
pub struct ModelHolder {
    inner: Box<dyn DefaultModel>,
}
impl ModelHolder {
    pub fn new(d_model: Box<dyn DefaultModel>) -> Self {
        Self { inner: d_model }
    }
    /// Return a `Model::default()`
    #[allow(dead_code)]
    pub fn get_model(&self) -> Box<dyn Model> {
        self.inner.default()
    }
}

/// `Model` is a public trait where all necessary functions
/// for a model should be implemented here.
pub trait Model {
    fn get_name(&self) -> &'static str {
        "GenericModel"
    }
    /// Initialize a `Model` with given `Data`. `Model` do not need a
    /// `new()` method but should be `Default` for factory pattern.
    fn init(&mut self, data: &Data) -> &mut dyn Model;
    /// Train the `Model`.
    fn train(&mut self) -> &mut dyn Model;
    /// Given one `Transaction`, predict the `Rating`.
    fn predict(&self, trans: &Transaction) -> Rating;
    /// By default it outputs to `<model-name>.txt` in the working directory.
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
