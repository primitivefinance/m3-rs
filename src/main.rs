/// External
use ethers::prelude::*;
use itertools_num::linspace;
use visualize::{design::*, plot::*};
/// Local
mod plots;
mod models;
use models::rmm_01::*;
use models::base_model::*;
use plots::*;

const RPC_URL: &str = "https://eth.llamarpc.com";

/// tokio
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let block_number: U64 = provider.get_block_number().await?;
    println!("{block_number}");

    // Instantiate the model with it's identifying parameters
    let mut fund = BaseModel::new(
        "RMM01".to_string(),
        "0.0.1".to_string(),
        "1".to_string(),
        "1".to_string(),
    );

    // Set it's objective using a chosen model and model parameters.
    fund.set_objective(Box::new(RMM01 {
        strike: 1_f64,
        volatility: 0.1_f64,
        time_to_maturity: 1.0,
    }));

    // This example plots the liquidity density over a range of prices, using visualize-rs and the functions in the plot.rs module.
    let price_start = 0.0_f64;
    let price_end = 20.0_f64;
    let number_of_prices = 1000;
    let prices = linspace(price_start, price_end, number_of_prices).collect::<Vec<f64>>();

    let display = Display {
        transparent: false,
        mode: DisplayMode::Light,
        show: false,
    };
    plot_liquidity_density(fund, display, prices);

    Ok(())
}
