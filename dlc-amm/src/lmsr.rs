//! Logarithmic market scoring rule

use std::f64::consts::E;

use crate::MarketScoringRule;



/// Multi dimensional cost function
pub(crate) fn cost_function_md(total_security: &[f64], b: f64) -> f64 {
    b * total_security.into_iter().map(|q| { E.powf(q/b) } ).sum::<f64>().ln()
}

pub(crate) fn price_for_purchase(total_security: &[f64], purchase_vector: &[f64], b: f64) -> f64 {
    let mut total_security_after = Vec::with_capacity(total_security.len());
    for (i,q) in total_security.iter().enumerate() {
        total_security_after[i] = *q + purchase_vector[i];
    }
    cost_function_md(total_security_after.as_ref(), b) - cost_function_md(total_security, b)
}

/// Price of the specific security at certain time.
/// This is an special case of `price_for_purchase` function to purchase
/// Infinitely small amount of security.
/// And it is a partial derivatives of the cost function.
pub(crate) fn price_for_showing(total_security: &[f64], security_index: usize, b: f64) -> f64 {
    let l = |q: &f64| { E.powf(q/b) };
    l(&total_security[security_index]) / total_security.iter().map(l).sum::<f64>()
}


pub struct LMSRScoringRule {
    total_securities: Vec<f64>,
    liquidity: f64
}

impl LMSRScoringRule {
    pub fn new(outcomes: usize, liquidity: f64) -> Self {
        Self {
            total_securities: Vec::with_capacity(outcomes),
            liquidity
        }
    }
}

impl MarketScoringRule for LMSRScoringRule {
    fn cost_function(&self) -> f64 {
        cost_function_md(&self.total_securities.as_ref(), self.liquidity)
    }

    fn price_for_purchase(&self, purchase_vector: &[f64]) -> f64 {
        price_for_purchase(&self.total_securities, purchase_vector, self.liquidity)
    }

    fn price_for_showing(&self, security_index: usize) -> f64 {
        price_for_showing(&self.total_securities, security_index, self.liquidity)
    }

    fn total_securities(&self) -> &[f64] {
        self.total_securities.as_ref()
    }
}
