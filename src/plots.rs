
use visualize::{design::*, plot::*};
use crate::models::base_model::BaseModel;

pub fn plot_liquidity_density(base_model: BaseModel, display: Display, prices: Vec<f64>) {
    let title: String = String::from("Liquidity Density");
    let x_coordinates = prices.clone(); // Prices
    let curve = Curve {
        x_coordinates: x_coordinates.clone(),
        y_coordinates: base_model.objective.unwrap().get_liquidity_density(prices).clone(),
        design: CurveDesign {
            color: Color::Green,
            color_slot: 1,
            style: Style::Lines(LineEmphasis::Light),
        },
        name: Some(format!("{} {}", "\\tau=", 1.0)),
    };

    // Capable of graphing multiple liquidity distributions, edit this code to do so.
    let curves = vec![curve];

    // Build the plot's axes
    if let Some(last_price) = x_coordinates.last() {
        let axes = Axes {
        x_label: String::from("S"),
        y_label: String::from("L(S)"),
        bounds: (vec![x_coordinates[0], *last_price], vec![0.0, 1.0]),
    };

    // Plot it.
    transparent_plot(Some(curves), None, axes, title, display, None);
    } else {
        println!("prices is empty");
    }
    
}