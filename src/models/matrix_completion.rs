use super::*;

#[derive(Debug)]
struct MatrixCompletion {
    customer_movie: Matrix<f64>,
}

impl Default for MatrixCompletion {
    fn default() -> Self {
        MatrixCompletion {
            customer_movie: Matrix::zeros(1, 1),
        }
    }
}

inventory::submit!(ModelHolder::new(Box::new(MatrixCompletion::default())));

impl Model for MatrixCompletion {
    fn get_name(&self) -> &'static str {
        "MatrixCompletion"
    }
    fn init(&mut self, data: &Data) -> &mut dyn Model {
        self.customer_movie = data.training_data_to_matrix();
        self
    }
    fn train(&mut self) -> &mut dyn Model {
        self
    }
    fn predict(&self, _trans: &Transaction) -> Rating {
        0
    }
}
