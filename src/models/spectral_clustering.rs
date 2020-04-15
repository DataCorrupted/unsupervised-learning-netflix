use super::*;

#[derive(Debug)]
struct SpectralClustering;

impl Default for SpectralClustering {
    fn default() -> Self {
        SpectralClustering
    }
}

inventory::submit!(ModelHolder::new(Box::new(SpectralClustering::default())));

impl Model for SpectralClustering {
    fn get_name(&self) -> &'static str {
        "SpectralClustering"
    }
    fn init(&mut self, data: &Data, metadata: &MetaData) -> &mut dyn Model {
        let MetaData {
            num_customers: n,
            num_movies: m,
            num_train: _,
            num_cross_valid: _,
            trans_freq: _,
            tests_freq: _,
        } = metadata;
        let (m, n) = (*m as usize, *n as usize);
        let mut customer_movie = Matrix::<f64>::zeros(n, m);
        data.train.iter().for_each(|t|{
            customer_movie[[t.customer_id, t.movie_id]] = t.rating as f64;
        });
        /* Todo: Should average rating be all voted rating or all rating. */
        let averge_rating: Vec<f64> = (0..n).into_iter().map(|idx| {
            let p = customer_movie.get_row(idx).unwrap().iter().fold((0.0, 0.0), |(s, t), &r|{
                if r != 0.0 {
                    (s + r, t + 1.0)
                } else {
                    (s, t)
                }
            });
            p.0 / p.1
        }).collect();
        let mut similarility = Matrix::<f64>::zeros(m,  m);

        /*Todo: Calculate similarity matrix*/

        self
    }
    fn train(&mut self) -> &mut dyn Model {
        self
    }
    fn predict(&self, _trans: &Transaction) -> Rating {
        0
    }
}
