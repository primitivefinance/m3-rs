# M3-rs

Constant function market makers are a class of automated market making strategies implemented via smart contracts on blockchain networks like Ethereum. 
M3-rs is a Rust interface for interacting with these models and their properties, such as the amount of assets that can be sold at a price.

# Install
```
Add to Cargo.toml:
m3-rs = { git = "https://github.com/primitivefinance/m3-rs" }
```

# Usage


```rust
use m3_rs::{...};

fn main() {
    // Create a base model
    let mut base = BaseModel::new(
        "base_model_name".to_string(),
        "base_model_version".to_string(),
        "base_model_code".to_string(),
        "base_model_id".to_string(),
    );

    // Set it's objective, i.e. the model.
    base.set_objective(Box::new(RMM01 {
        strike: 1_f64,
        volatility: 1_f64,
        time_to_maturity: 1.0,
    }))

    // Use the functions in plot.rs to plot different derived data using these models.
} 
```