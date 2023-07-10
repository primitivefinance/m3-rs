/// math
use visualize::functions::*;
/// models
use crate::models::base_model::*;


/// Parameters for the RMM01 model.
pub struct RMM01 {
    pub strike: f64,
    pub volatility: f64,
    pub time_to_maturity: f64,
}

impl Objective for RMM01 {
    fn describe(&self) -> String {
        format!(
            "RMM01: strike: {}, volatility: {}, time_to_maturity: {}",
            self.strike, self.volatility, self.time_to_maturity
        )
    }

    fn get_liquidity_density(&self, prices: Vec<f64>) -> Vec<f64> {
        liquidity_distribution(prices, self.strike, self.volatility, self.time_to_maturity)
    }

    fn get_reported_price(&self) -> f64 {
        todo!()
    }

    fn get_virtual_price_range(&self) -> PriceRange {
        PriceRange {
            lower: 0.0,
            upper: 0.0,
        }
    }
}

/// Fetches the liquidity distribution of a pool that uses the RMM01 model.
/// Returns a vector of x and y coordinates.
fn liquidity_distribution(prices: Vec<f64>, strike: f64, volatility: f64, tau: f64) -> Vec<f64> {
    let mut y_coordinates = Vec::new(); // Liquidity densities

    // Compute the liquidity density at each price
    // Probability density function = f(x)
    // f(d_1), where d1 = (ln(S/K) + (r + σ^2/2)τ) / (σ√τ)
    let pdf_of_d_one = standard_gaussian_pdf(d_one(prices.clone(), strike, volatility, tau));

    // f(d_1) / σ√τ
    let density_over_vol = pdf_of_d_one
        .iter()
        .map(|output| output / (volatility * tau.sqrt()))
        .collect::<Vec<f64>>();

    // For each price, compute the liquidity density
    // Liquidity density = f(d_1) / σ√τ / S
    for (i, y) in density_over_vol.iter().enumerate() {
        y_coordinates.push(y / prices[i]);
    }

    // Return the coordinates of the liquidity distribution
    y_coordinates
}
