use super::*;

#[derive(Default, Debug)]
struct MatrixCompletion;

inventory::submit!(ModelHolder::new(Box::new(MatrixCompletion::default())));

impl Model for MatrixCompletion {
    fn get_name(&self) -> &'static str {
        "MatrixCompletion"
    }
    fn init(&mut self, _data: &Data, _metadata: &MetaData) -> &mut dyn Model {
        self
    }
    fn train(&mut self) -> &mut dyn Model {
        self
    }
    fn predict(&self, _trans: &Transaction) -> Rating {
        0
    }
}
