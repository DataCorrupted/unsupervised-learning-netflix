use super::*;

#[allow(dead_code)]
#[derive(Default, Debug)]
struct MatrixCompletion;

#[doc(hidden)]
inventory::submit!(ModelHolder::new(Box::new(MatrixCompletion)));

impl Model for MatrixCompletion {
    fn get_name(&self) -> &'static str {
        "MatrixCompletion"
    }
    fn init(&mut self, _data: &Data) -> &mut dyn Model {
        self
    }
    fn train(&mut self) -> &mut dyn Model {
        self
    }
    fn predict(&self, _trans: &Transaction) -> Rating {
        0
    }
}
