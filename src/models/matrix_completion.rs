use super::*;

#[allow(dead_code)]
struct MatrixCompletion;

impl Models for MatrixCompletion {
    const NAME: &'static str = "MatrixCompletion";
    fn predict(&self, _trans: &Transaction) -> Rating {
        unimplemented!()
    }
}
