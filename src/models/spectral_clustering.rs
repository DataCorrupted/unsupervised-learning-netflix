use super::*;

#[allow(dead_code)]
struct SpectralClustering;

impl Models for SpectralClustering {
    const NAME: &'static str = "SpectralClustering";
    fn predict(&self, _trans: &Transaction) -> Rating {
        unimplemented!()
    }
}
