use alloy::primitives::{Address, U256, address};
use std::collections::hash_map::HashMap;
use std::env;
use std::sync::LazyLock;

pub const VOTING_CONTRACT_ADDRESS: &str = "0xdD5CB392A549644295862f96f25484a56FB2e6a8";
pub const INACTIVE_ASSETS: [Address; 1] = [address!("0x576e2bed8f7b46d34016198911cdf9886f78bea7")];
pub const JOOCE_INT_WEIGHT: u16 = 1311;

pub static CHAIN_ID_TO_URL: LazyLock<HashMap<U256, String>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(
        U256::from(1151111081099710_u64),
        env::var("SOLANA_RPC").unwrap(),
    );
    map.insert(U256::from(8453), env::var("BASE_RPC").unwrap());
    map.insert(U256::from(1), env::var("ETHEREUM_RPC").unwrap());
    map.insert(U256::from(56), env::var("BINANCE_RPC").unwrap());
    map.insert(U256::from(43114), env::var("AVALANCHE_RPC").unwrap());
    map.insert(U256::from(10), env::var("OPTIMISM_RPC").unwrap());
    map.insert(U256::from(42161), env::var("ARBITRUM_RPC").unwrap());

    map
});

pub static CHAIN_ID_TO_STRING: LazyLock<HashMap<U256, &str>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(U256::from(1151111081099710_u64), "SOLANA");
    map.insert(U256::from(8453), "BASE");
    map.insert(U256::from(1), "ETHEREUM");
    map.insert(U256::from(56), "BSC");
    map.insert(U256::from(43114), "AVALANCHE");
    map.insert(U256::from(10), "OPTIMISM");
    map.insert(U256::from(42161), "ARBITRUM");

    map
});

pub static ADDR_TO_SOL_MINT_ADDR: LazyLock<HashMap<Address, &str>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(
        address!("0x6A851667B20800988c0cE34276F63f86f085BB2c"),
        "MEW1gQWJ3nEXg2qgERiKu7FAFj79PHvQVREQUzScPP5",
    );
    map.insert(
        address!("0xaf78C51362ee75477Aa11fc660d1955dD34F37B8"),
        "2qEHjDLDLbuBgRYvsxhc5D6uDWAivNFZGan56P1tpump",
    );
    map.insert(
        address!("0xD29E4552ed325ab75A99cC661c280625D5B38cE9"),
        "9BB6NFEcjBCtnNLFko2FqVQBq8HHM13kCyYcdQbgpump",
    );
    map.insert(
        address!("0xC069D48749327243b699C1B91D22613DC39551e4"),
        "EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm",
    );
    map.insert(
        address!("0xA48F7855a0b3200B1d0CA84c12399171dD456624"),
        "2zMMhcVQEXDtdE6vsFS7S7D5oUodfJHE8vd1gnBouauv",
    );
    map.insert(
        address!("0xf9a337194f9278275Ee28CeFeb58adAdBcB62572"),
        "7GCihgDB8fe6KNjn2MYtkzZcRjQy3t9GHdC8uHYmW2hr",
    );
    map.insert(
        address!("0x4520A52CfB5daD1a6aeAd5f43C96eD2A9760E77e"),
        "HeLp6NuQkmYB4pYWo2zYs22mESHXPQYzXbB8n4V98jwC",
    );
    map.insert(
        address!("0x2107895Cd820573Cdc943dF2A26910945555C29d"),
        "63LfDmNb3MQ8mw9MtZ2To9bEA2M71kZUUGq5tiJxcqj9",
    );
    map.insert(
        address!("0xf3BC9c7DaA0aeb694c95d65c2FE86137695F2a59"),
        "ED5nyyWEzpPPiWimP8vYm7sD7TD3LAt3Q3gRTWHzPJBY",
    );
    map.insert(
        address!("0x75939e0A1Eb2321DBaCcDB2E637DdBa29098Eb16"),
        "CzLSujWBLFsSjncfkh59rUFqvafWcY5tzedWJSuypump",
    );
    map.insert(
        address!("0xDD2DDf33d0936A2A4E8316DFc89F538fCc1fA5b1"),
        "ukHH6c7mMyiWCf1b9pnWe25TSpkDDt3H5pQZgZ74J82",
    );
    map.insert(
        address!("0xfFeBa30f39FaA3601911090d5ce7388D719109c9"),
        "A8C3xuqscfmyLrte3VmTqrAq8kgMASius9AFNANwpump",
    );
    map.insert(
        address!("0xa697e272a73744b343528c3bc4702f2565b2f422"),
        "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263",
    );

    map
});
