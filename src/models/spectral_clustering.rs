use super::*;

#[allow(dead_code)]
#[derive(Default, Debug)]
struct SpectralClustering;

#[doc(hidden)]
inventory::submit!(ModelHolder::new(Box::new(SpectralClustering)));

impl Model for SpectralClustering {
    fn get_name(&self) -> &'static str {
        "SpectralClustering"
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
