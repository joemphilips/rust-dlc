
use crate::{lmsr::{cost_function_md, price_for_purchase, price_for_showing}, MarketScoringRule};


pub struct LSLMSRScoringRule {
    total_securities: Vec<f64>,
    alpha: f64
}

impl LSLMSRScoringRule {
    pub fn new(outcomes: usize, alpha: f64) -> Self {
        Self {
            total_securities: Vec::with_capacity(outcomes),
            alpha
        }
    }

    pub fn b(&self) -> f64 {
        &self.alpha * &self.total_securities.iter().sum::<f64>()
    }
}


impl MarketScoringRule for LSLMSRScoringRule {
    fn cost_function(&self) -> f64 {
        cost_function_md(&self.total_securities, self.b())
    }

    fn price_for_purchase(&self, purchase_vector: &[f64]) -> f64 {
        price_for_purchase(&self.total_securities, purchase_vector, self.b())
    }

    fn price_for_showing(&self, security_index: usize) -> f64 {
        price_for_showing(&self.total_securities, security_index, self.b())
    }
    fn total_securities(&self) -> &[f64] {
        self.total_securities.as_ref()
    }
}
