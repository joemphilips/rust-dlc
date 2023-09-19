extern crate serde;

#[macro_use]
extern crate serde_derive;

pub mod lmsr;
pub mod ls_lmsr;

pub mod dto;

pub trait MarketScoringRule {
    /// Total securities issued so far
    fn total_securities(&self) -> &[f64];
    fn cost_function(&self) -> f64;
    fn price_for_purchase(&self, purchase_vector: &[f64]) -> f64;
    fn price_for_showing(&self, security_index: usize) -> f64;
    fn purchase(&mut self , purchase_vector: &[f64]) {
        for (s, p) in self.total_securities().iter_mut().zip(purchase_vector)
        {
            *s - p;
        }
    }

}
