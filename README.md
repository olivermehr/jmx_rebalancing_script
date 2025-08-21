# jmx-weights

This repository contains a Rust utility that queries the **Jooce Voting** contract to gather current asset weights, normalises them and exports the result to a Google Sheet for portfolio rebalancing.

## Setup

1. Install [Rust](https://www.rust-lang.org/).
2. Create a `.env` file with the required credentials and RPC endpoints:

```
PRIVATE_KEY=...        # key used to query the contract
SPREADSHEET_ID=...
SOLANA_RPC=...
BASE_RPC=...
ETHEREUM_RPC=...
BINANCE_RPC=...
AVALANCHE_RPC=...
OPTIMISM_RPC=...
ARBITRUM_RPC=...
```

3. Place your Google service account credentials in `jooce-cred.json` so the script can write to the sheet.

## Usage

```bash
# Fetch weights and append a new worksheet with the data
cargo run

# Optionally checkpoint asset weights on-chain before fetching
cargo run -- update
```

The program prints the computed weight map to stdout and adds a timestamped sheet containing:
- token symbol
- percentage of the total weight
- `u16` representation used on-chain
- chain identifier

## Testing

There are currently no dedicated unit tests. Running `cargo test` will compile the project and verify that all dependencies build correctly.
