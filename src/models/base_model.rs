use std::fmt::Display;

/// Abstraction for a pool of capital managed by a smart contract.
///
/// For autonomous market making smart contracts, the pools
/// are responsible for holding the assets and executing trades.
/// These pools have implied properties that this abstraction will expose.
///
/// * `name` - The human readable name of the pool.
/// * `version` - The version of the pool.
/// * `code` - A ticker to identify the pool short hand.
/// * `id` - A unique pool identifier.
pub struct BaseModel {
    pub name: String,
    pub version: String,
    pub code: String,
    pub id: String,
    pub objective: Option<Box<dyn Objective>>,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct PriceRange {
    pub lower: f64,
    pub upper: f64,
}

pub trait Objective {
    fn describe(&self) -> String;
    fn get_reported_price(&self) -> f64;
    fn get_liquidity_density(&self, prices: Vec<f64>) -> Vec<f64>;
    fn get_virtual_price_range(&self) -> PriceRange;
    fn get_trading_curve(&self, prices: Vec<f64>, scaling: Option<f64>) -> (Vec<f64>, Vec<f64>);
}

pub trait ObjectiveDisplay {
    fn plot_liquidity_density(&self, display: dyn Display, prices: Vec<f64>);
}

/// Implementing the class
impl BaseModel {
    pub fn new(name: String, version: String, code: String, id: String) -> BaseModel {
        BaseModel {
            name,
            version,
            code,
            id,
            objective: None,
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn set_objective(&mut self, objective: Box<dyn Objective>) {
        self.objective = Some(objective);
    }
}

impl Default for BaseModel {
    fn default() -> Self {
        BaseModel {
            name: "default".to_string(),
            version: "default".to_string(),
            code: "default".to_string(),
            id: "default".to_string(),
            objective: None,
            x: 0.0,
            y: 0.0,
        }
    }
}
