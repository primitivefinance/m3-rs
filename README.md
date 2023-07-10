# af-rs

Interact with the Portfolio protocol using Rust models to abstract the underlying pools.


## What we want:
Given a uniswap pool with two tokens and a fee tier, export the reserves/liquidity at each tick into a csv
Given a curve pool with x tokens, export the reserves and compute its liquidity density
Given a primitive pool with two tokens and params compute its effective price range and liquidity density

## What do I really want to do?
I want to load a pool into memory so I can analyze it. To analyze it, I want to see its price, effective price range, liquidity distribution, arb bounds, health, etc. Ideally I can export data into a simple format, and also use our visualize-rs tool to render the pool into a nice graph.

To do this, I need to have a clean interface for loading a pool from the chain into memory in a rust data structure.


Todo: I want to graph the liquidity density from the chain.