/// Matrix completion.
pub mod matrix_completion;
/// Spectral clustering.
pub mod spectral_clustering;

use elapsed::measure_time;
use log::*;
use std::fmt::Debug;

use nalgebra::core::DMatrix;

use crate::data::*;

/// `DefaultModel` generate uninitialized `Model`.
pub trait DefaultModel: Model {
    fn default_model(&self) -> Box<dyn Model>;
}
impl<T> DefaultModel for T
where
    T: Model + Default + 'static,
{
    fn default_model(&self) -> Box<dyn Model> {
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
        self.inner.default_model()
    }
    pub fn get_name(&self) -> &'static str {
        self.inner.get_name()
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
    fn predict_all(&self, test_data: &Vec<Transaction>) -> Vec<Rating> {
        test_data.iter().map(|t| self.predict(t)).collect()
    }
}

inventory::collect!(ModelHolder);
