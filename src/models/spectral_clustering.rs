use super::*;

use nalgebra::linalg::SymmetricEigen;

#[derive(Debug)]
struct SpectralClustering {
    movie_similarity: DMatrix<f64>,
    customer_similarity: DMatrix<f64>,
    customer_movie: DMatrix<f64>,
}

impl Default for SpectralClustering {
    fn default() -> Self {
        SpectralClustering {
            movie_similarity: DMatrix::zeros(1, 1),
            customer_similarity: DMatrix::zeros(1, 1),
            customer_movie: DMatrix::zeros(1, 1),
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
                self.customer_movie = data.training_data_to_matrix().columns(0, 1000).into();

                //self.customer_movie = data.training_data_to_matrix()
            });
            info!("Convert data to matrix finished... elapsed: {}", elapsed);
            info!("Matrix shape: {:?}", self.customer_movie.shape());

            info!("Generate movie similarity matrix");
            let (elapsed, _) = measure_time(|| {
                self.movie_similarity = self.customer_movie.get_similarity_matrix()
            });
            info!(
                "Generate movie similarity matrix finished... elapsed: {}",
                elapsed
            );

            /*
            info!("Generate customer similarity matrix");
            let (elapsed, _) = measure_time(|| {
                self.customer_similarity = self.customer_movie.transpose().get_similarity_matrix();
            });
            info!(
                "Generate customer similarity matrix finished... elapsed: {}",
                elapsed
            );
            */
        });
        info!(
            "{}.init(&Data) finished... elapsed: {}",
            self.get_name(),
            elapsed
        );
        self
    }
    fn train(&mut self) -> &mut dyn Model {
        info!("{}.train()", self.get_name());
        let (elapsed, (movie_eigen, customer_eigen)) = measure_time(|| {
            info!("Eigen decompose movie similarity matrix");
            let (elapsed, movie_eigen) =
                measure_time(|| SymmetricEigen::new(self.movie_similarity.clone()));
            info!(
                "Eigen decompose movie similarity matrix finished... elapsed: {}",
                elapsed
            );

            info!("Eigen decompose customer similarity matrix");
            let (elapsed, customer_eigen) =
                measure_time(|| SymmetricEigen::new(self.movie_similarity.clone()));
            info!(
                "Eigen decompose customer similarity matrix finished... elapsed: {}",
                elapsed
            );
            (movie_eigen, customer_eigen)
        });
        info!(
            "{}.train() finished... elapsed: {}",
            self.get_name(),
            elapsed
        );
        let cnt = movie_eigen.eigenvalues.iter().fold(0, |cnt, v| {
            if (v - 0f64).abs() < 1e-10 {
                cnt + 1
            } else {
                cnt
            }
        });
        info!("# of 0 eigen values in movie: {}", cnt);

        let cnt = customer_eigen.eigenvalues.iter().fold(0, |cnt, v| {
            if (v - 0f64).abs() < 1e-10 {
                cnt + 1
            } else {
                cnt
            }
        });
        info!("# of 0 eigen values in customer: {}", cnt);

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
trait PearsonCosineSimilarity {
    /// Get a vector over all rows(items)
    fn get_avg_and_non_zero_idx(&self) -> (Vec<f64>, Vec<Vec<usize>>);

    /// Get similarity matrix of size m x m from matrix of size m x n
    fn get_similarity_matrix(&self) -> DMatrix<f64>;
}

impl PearsonCosineSimilarity for DMatrix<f64> {
    fn get_avg_and_non_zero_idx(&self) -> (Vec<f64>, Vec<Vec<usize>>) {
        let (n, m) = self.shape();
        let mut non_zero_idx = vec![vec![]; m];
        let mut avg = vec![0f64; m];

        for j in 0..m {
            let curr = &mut non_zero_idx[j];
            let col_j = self.column(j);
            let mut sum = 0f64;
            for i in 0..n {
                if col_j[i] != 0f64 {
                    curr.push(i);
                    sum += col_j[i];
                }
            }
            avg[j] = if curr.len() != 0 {
                sum / curr.len() as f64
            } else {
                0f64
            };
        }
        (avg, non_zero_idx)
    }
    fn get_similarity_matrix(&self) -> DMatrix<f64> {
        let (_, m) = self.shape();
        let (avg, non_zero_idx) = self.get_avg_and_non_zero_idx();

        let mut norm = vec![0f64; m];
        let mut matrix = self.clone();
        for j in 0..non_zero_idx.len() {
            non_zero_idx[j].iter().for_each(|&i| {
                matrix[(i, j)] -= avg[j];
                norm[j] += matrix[(i, j)].powi(2);
            });
        }
        norm.iter_mut().for_each(|n| *n = n.sqrt());

        let mut similarility = Self::zeros(m, m);
        // Divide similarity[i][j] by |item_i| and |item_j|, which is located
        // in the diag of similarity.
        for j in 0..m {
            for i in j + 1..m {
                if norm[i] == 0f64 || norm[j] == 0f64 {
                    continue;
                }
                let nx = &non_zero_idx[i];
                let ny = &non_zero_idx[j];
                let mut p = 0;
                let mut q = 0;
                while p < nx.len() && q < ny.len() {
                    if nx[p] == ny[q] {
                        similarility[(i, j)] += matrix[(nx[p], i)] * matrix[(ny[q], j)];
                        p += 1;
                        q += 1;
                    } else if nx[p] < ny[q] {
                        p += 1;
                    } else {
                        q += 1;
                    }
                }
                similarility[(i, j)] = similarility[(i, j)] / (norm[i] * norm[j]);
                similarility[(j, i)] = similarility[(i, j)];
            }
        }
        similarility
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_pearson_cosine_similarity() {
        let matrix = DMatrix::<f64>::from_row_slice(
            3,
            6,
            &[
                1f64, 2f64, 0f64, 0f64, 1f64, 0f64, 0f64, 1f64, 2f64, 0f64, 3f64, 0f64, 2f64, 2f64,
                4f64, 0f64, 2f64, 0f64,
            ],
        );
        assert!(matrix.shape() == (3, 6));
        let (avg, non_zero_idx) = matrix.get_avg_and_non_zero_idx();
        assert!(avg == vec![1.5f64, 5f64 / 3f64, 3f64, 0f64, 2f64, 0f64]);
        assert!(
            non_zero_idx
                == vec![
                    vec![0, 2],
                    vec![0, 1, 2],
                    vec![1, 2],
                    vec![],
                    vec![0, 1, 2],
                    vec![],
                ]
        );
        let similarity = matrix.get_similarity_matrix();
        assert!(similarity == similarity.transpose());
        assert!(similarity[(0, 0)] == 0f64);
        assert!(similarity[(5, 3)] == 0f64);
        assert!(similarity[(0, 1)] == 0f64);
        assert!(similarity[(3, 0)] == 0f64);
        assert!(similarity[(4, 0)] - 0.5f64 < 1e-10);
        assert!((similarity[(1, 4)] - -f64::sqrt(3f64) / 2f64).abs() < 1e-10);
    }
}
