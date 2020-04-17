use super::*;

#[derive(Debug)]
struct SpectralClustering {
    movie_similarity: Matrix<f64>,
    customer_similarity: Matrix<f64>,
    customer_movie: Matrix<f64>,
}

impl Default for SpectralClustering {
    fn default() -> Self {
        SpectralClustering {
            movie_similarity: Matrix::zeros(1, 1),
            customer_similarity: Matrix::zeros(1, 1),
            customer_movie: Matrix::zeros(1, 1),
        }
    }
}

inventory::submit!(ModelHolder::new(Box::new(SpectralClustering::default())));

impl Model for SpectralClustering {
    fn get_name(&self) -> &'static str {
        "SpectralClustering"
    }
    fn init(&mut self, data: &Data) -> &mut dyn Model {
        info!("{}.init(&Data)", self.get_name());
        let (elapsed, _) = measure_time(|| {
            info!("Convert data to matrix");
            let (elapsed, _) = measure_time(|| {
                self.customer_movie = data.training_data_to_matrix();
            });
            info!("Convert data to matrix finished... elapsed: {}", elapsed);
            
            info!("Generate movie similarity matrix");
            let (elapsed, _) = measure_time(|| {
                self.movie_similarity = self.customer_movie.get_similarity_matrix();
            });
            info!("Generate movie similarity matrix finished... elapsed: {}", elapsed);

            info!("Generate customer similarity matrix");
            let (elapsed, _) = measure_time(|| {
                self.customer_similarity = self.customer_movie.transpose().get_similarity_matrix();
            });
            info!("Generate customer similarity matrix finished... elapsed: {}", elapsed);
        });
        info!("{}.init(&Data) finished... elapsed: {}", self.get_name(), elapsed);
        self
    }
    fn train(&mut self) -> &mut dyn Model {
        info!("{}.train()", self.get_name());
        let (elapsed, (movie_eigen, customer_eigen)) = measure_time(|| {
            info!("Eigen decompose movie similarity matrix");
            let (elapsed, movie_eigen) = measure_time(|| {
                self.movie_similarity.eigendecomp()
            });
            info!("Eigen decompose movie similarity matrix finished... elapsed: {}", elapsed);

            info!("Eigen decompose customer similarity matrix");
            let (elapsed, customer_eigen) = measure_time(|| {
                self.customer_similarity.eigendecomp()
            });
            info!("Eigen decompose customer similarity matrix finished... elapsed: {}", elapsed);
            (movie_eigen, customer_eigen)
        });
        info!("{}.train() finished... elapsed: {}", self.get_name(), elapsed);
        let cnt = movie_eigen.unwrap().0.iter().fold(0, |cnt, v| {
            if (v - 0f64).abs() < 1e-10 {
                cnt + 1
            } else {
                cnt
            }
        });
        info!("# of 0 eigen values in movie: {}", cnt);
        let cnt = customer_eigen.unwrap().0.iter().fold(0, |cnt, v| {
            if (v - 0f64).abs() < 1e-10 {
                cnt + 1
            } else {
                cnt
            }
        });
        info!("# of 0 eigen values in movie: {}", cnt);

        self
    }
    fn predict(&self, _trans: &Transaction) -> Rating {
        0
    }
}

/// Get pearson consine similarity matrix of size m x m from matrix n x m;
/// 
/// Pearson consine similarity is defined by
/// ```math
/// cos(x, y) = \frac{ 
///     (x - \bar{x})^T \cdot (y - \bar{y}) 
/// }{
///     ||x - \bar{x}|| \cdot ||y - \bar{y}||
/// }
/// ```
trait PearsonCosineSimilarity: BaseMatrix<f64> {
    /// Get a vector over all rows(items)
    fn get_avg_vec_and_non_zero_idx(&self) -> (Vec<f64>, Vec<Vec<usize>>);

    /// Get similarity matrix of size m x m from matrix of size m x n
    fn get_similarity_matrix(&self) -> Matrix<f64>;
}

impl PearsonCosineSimilarity for Matrix<f64> {
    fn get_avg_vec_and_non_zero_idx(&self) -> (Vec<f64>, Vec<Vec<usize>>){
        let mut non_zero_idx = vec![];
        let mut avg = vec![];
        let matrix = self.transpose();
        for i in 0..matrix.rows() {
            non_zero_idx.push(vec![]);
            let last = &mut non_zero_idx.last_mut().unwrap();
            let row = matrix.get_row(i).unwrap();
            let mut sum = 0f64;
            for j in 0..matrix.cols() {
                if row[j] != 0f64 {
                    last.push(j);
                    sum += row[j];
                }
            }
            avg.push(
                if last.len() != 0 {
                    sum / last.len() as f64
                } else {
                    0f64
                }
            );
        }
        (avg, non_zero_idx)
    }
    fn get_similarity_matrix(&self) -> Matrix<f64> {
        let m = self.cols();
        let (avg, non_zero_idx) = self.get_avg_vec_and_non_zero_idx();
        let mut matrix = self.clone();
        for j in 0..non_zero_idx.len() {
            non_zero_idx[j].iter().for_each(|&i|{
                matrix[[i, j]] -= avg[j];
            });
        }
        // similarity[i][j] is cross product of item i and j
        let mut similarility = matrix.transpose() * matrix;
        // similarity[i][i] is |item_i|^2, let's make it |item_i|.
        for i in 0..m {
            similarility[[i, i]] = similarility[[i, i]].sqrt();
        }
        // Divide similarity[i][j] by |item_i| and |item_j|, which is located
        // in the diag of similarity.
        for i in 0..m {
            if avg[i] == 0f64 { continue; }
            for j in i + 1..m {
                if avg[j] == 0f64 {
                    continue;
                }
                let div = similarility[[i, i]] * similarility[[j, j]];
                similarility[[i, j]] /= div;
                similarility[[j, i]] = similarility[[i, j]];
            }
        }
        // Remove self loop im similarity[i][i].
        for i in 0..m {
            similarility[[i, i]] = 0f64;
        }
        similarility
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_pearson_cosine_similarity() {
        let matrix = Matrix::<f64>::new(
            3,
            6,
            vec![
                1f64, 2f64, 0f64, 0f64, 1f64, 0f64,
                0f64, 1f64, 2f64, 0f64, 3f64, 0f64,
                2f64, 2f64, 4f64, 0f64, 2f64, 0f64,
            ],
        );
        assert!(matrix.cols() == 6);
        assert!(matrix.rows() == 3);
        let (avg, non_zero_idx) = matrix.get_avg_vec_and_non_zero_idx();
        assert!(avg == vec![1.5f64, 5f64 / 3f64, 3f64, 0f64, 2f64, 0f64]);
        assert!(non_zero_idx == vec![
            vec![0, 2],
            vec![0, 1, 2],
            vec![1, 2],
            vec![],
            vec![0, 1, 2],
            vec![],
        ]);
        let similarity = matrix.get_similarity_matrix();
        assert!(similarity == similarity.transpose());
        assert!(similarity[[0, 0]] == 0f64);
        assert!(similarity[[5, 3]] == 0f64);
        assert!(similarity[[0, 1]] == 0f64);
        assert!(similarity[[3, 0]] == 0f64);
        assert!(similarity[[4, 0]] - 0.5f64 < 1e-10);
        assert!((similarity[[1, 4]] - -f64::sqrt(3f64) / 2f64).abs() < 1e-10);
    }
}
