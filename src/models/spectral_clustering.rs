use super::*;

#[derive(Debug)]
struct SpectralClustering {
    similarity: Matrix<f64>,
    customer_movie: Matrix<f64>,
}

impl Default for SpectralClustering {
    fn default() -> Self {
        SpectralClustering {
            similarity: Matrix::zeros(1, 1),
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
        let (elapsed, _) = measure_time(|| {
            self.customer_movie = data.training_data_to_matrix();
            self.similarity = self.customer_movie.transpose().get_similarity_matrix();
        });
        info!("{}.init(&Data) finished... elapsed: {}", self.get_name(), elapsed);
        self
    }
    fn train(&mut self) -> &mut dyn Model {
        let (elapsed, result) = measure_time(|| {
            self.similarity.eigendecomp()
        });
        info!("{}.train() finished... elapsed: {}", self.get_name(), elapsed);
        let (eig_val, _) = result.unwrap();
        let cnt = eig_val.iter().fold(0, |cnt, v| {
            if (v - 0f64).abs() < 1e-10 {
                cnt + 1
            } else {
                cnt
            }
        });
        info!("# of 0 eigen values: {}", cnt);
        self
    }
    fn predict(&self, _trans: &Transaction) -> Rating {
        0
    }
}

trait PearsonCosineSimilarity: BaseMatrix<f64> {
    /// Calculate pearson cosine between two vectors
    fn cosine(&self, i: usize, j: usize, avg: &Vec<f64>) -> f64 {
        assert!(i < self.rows());
        assert!(j < self.rows());

        let (dot, norm_i, norm_j) = self
            .get_row(i)
            .unwrap()
            .iter()
            .zip(self.get_row(j).unwrap().iter())
            .fold(
                (0f64, 0f64, 0f64),
                |(dot, norm_i, norm_j), (&ri_k, &rj_k)| {
                    let rika = if ri_k == 0f64 { 0f64 } else { ri_k - avg[i] };
                    let rjka = if rj_k == 0f64 { 0f64 } else { rj_k - avg[j] };
                    (
                        dot + rika * rjka,
                        norm_i + rika.powi(2),
                        norm_j + rjka.powi(2),
                    )
                },
            );
        if norm_i == 0f64 || norm_j == 0f64 {
            0f64
        } else {
            dot / (norm_i * norm_j).sqrt()
        }
    }

    /// Get a vector over all rows(items)
    fn get_avg_vec(&self) -> Vec<f64> {
        self.iter_rows()
            .map(|row| {
                let (sum, tot) = row.iter().fold((0f64, 0f64), |(sum, tot), &row_k| {
                    if row_k == 0f64 {
                        (sum, tot)
                    } else {
                        (sum + row_k, tot + 1f64)
                    }
                });
                if tot == 0f64 {
                    0f64
                } else {
                    sum / tot
                }
            })
            .collect()
    }

    /// Get similarity matrix of size m x m from matrix of size m x n
    fn get_similarity_matrix(&self) -> Matrix<f64> {
        let m = self.rows();
        let mut similarility = Matrix::<f64>::zeros(m, m);
        let avg = self.get_avg_vec();
        for i in 0..m {
            for j in 0..m {
                similarility[[i, j]] = if i > j {
                    similarility[[j, i]]
                } else if j == i {
                    1f64
                } else {
                    self.cosine(i, j, &avg)
                };
            }
        }
        similarility
    }
}

impl PearsonCosineSimilarity for Matrix<f64> {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_pearson_cosine_similarity() {
        let matrix = Matrix::<f64>::new(
            6,
            3,
            vec![
                1f64, 0f64, 2f64, 2f64, 1f64, 2f64, 0f64, 2f64, 4f64, 0f64, 0f64, 0f64, 1f64, 3f64,
                2f64, 0f64, 0f64, 0f64,
            ],
        );
        assert!(matrix.cols() == 3);
        assert!(matrix.rows() == 6);
        let avg = matrix.get_avg_vec();
        assert!(avg == vec![1.5f64, 5f64 / 3f64, 3f64, 0f64, 2f64, 0f64]);
        let similarity = matrix.get_similarity_matrix();
        assert!(similarity == similarity.transpose());
        assert!(similarity[[0, 0]] == 1f64);
        assert!(similarity[[5, 3]] == 0f64);
        assert!(similarity[[0, 1]] == 0f64);
        assert!(similarity[[3, 0]] == 0f64);
        assert!(similarity[[4, 0]] - 0.5f64 < 1e-10);
        assert!((similarity[[1, 4]] - -f64::sqrt(3f64) / 2f64).abs() < 1e-10);
    }
}
