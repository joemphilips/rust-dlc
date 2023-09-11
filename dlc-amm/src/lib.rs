pub mod lmsr;
pub mod ls_lmsr;


pub trait MarketScoringRule {
    fn cost_function(&self) -> f64;
    fn price_for_purchase(&self, purchase_vector: &[f64]) -> f64;
    fn price_for_showing(&self, security_index: usize) -> f64;
}
