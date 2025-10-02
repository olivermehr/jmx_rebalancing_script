use alloy::primitives::{Address, U256, address};
use solana_sdk::pubkey::Pubkey;
use std::{collections::hash_map::HashMap, env, str::FromStr, sync::LazyLock};

pub const VOTING_CONTRACT_ADDRESS: &str = "0xdD5CB392A549644295862f96f25484a56FB2e6a8";
pub const INACTIVE_ASSETS: [Address; 1] = [address!("0x576e2bed8f7b46d34016198911cdf9886f78bea7")];
pub const JOOCE_INT_WEIGHT: u16 = 1311;
pub const SOLANA_CHAIN_ID: U256 = U256::from_limbs([1151111081099710_u64, 0, 0, 0]);
pub const MIN_RELATIVE_WEIGHT: f64 = 0.005;
pub const SCALE: u128 = 10_000_000_000u128 * 1e18 as u128;

pub static CHAIN_ID_TO_URL: LazyLock<HashMap<U256, String>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(SOLANA_CHAIN_ID, env::var("SOLANA_RPC").unwrap());
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
    map.insert(SOLANA_CHAIN_ID, "SOLANA");
    map.insert(U256::from(8453), "BASE");
    map.insert(U256::from(1), "ETHEREUM");
    map.insert(U256::from(56), "BSC");
    map.insert(U256::from(43114), "AVALANCHE");
    map.insert(U256::from(10), "OPTIMISM");
    map.insert(U256::from(42161), "ARBITRUM");

    map
});

pub static ADDR_TO_SOL_MINT_ADDR: LazyLock<HashMap<Address, Pubkey>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(
        address!("0x6A851667B20800988c0cE34276F63f86f085BB2c"),
        Pubkey::from_str("MEW1gQWJ3nEXg2qgERiKu7FAFj79PHvQVREQUzScPP5").unwrap(),
    );
    map.insert(
        address!("0xaf78C51362ee75477Aa11fc660d1955dD34F37B8"),
        Pubkey::from_str("2qEHjDLDLbuBgRYvsxhc5D6uDWAivNFZGan56P1tpump").unwrap(),
    );
    map.insert(
        address!("0xD29E4552ed325ab75A99cC661c280625D5B38cE9"),
        Pubkey::from_str("9BB6NFEcjBCtnNLFko2FqVQBq8HHM13kCyYcdQbgpump").unwrap(),
    );
    map.insert(
        address!("0xC069D48749327243b699C1B91D22613DC39551e4"),
        Pubkey::from_str("EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm").unwrap(),
    );
    map.insert(
        address!("0xA48F7855a0b3200B1d0CA84c12399171dD456624"),
        Pubkey::from_str("2zMMhcVQEXDtdE6vsFS7S7D5oUodfJHE8vd1gnBouauv").unwrap(),
    );
    map.insert(
        address!("0xf9a337194f9278275Ee28CeFeb58adAdBcB62572"),
        Pubkey::from_str("7GCihgDB8fe6KNjn2MYtkzZcRjQy3t9GHdC8uHYmW2hr").unwrap(),
    );
    map.insert(
        address!("0x4520A52CfB5daD1a6aeAd5f43C96eD2A9760E77e"),
        Pubkey::from_str("HeLp6NuQkmYB4pYWo2zYs22mESHXPQYzXbB8n4V98jwC").unwrap(),
    );
    map.insert(
        address!("0x2107895Cd820573Cdc943dF2A26910945555C29d"),
        Pubkey::from_str("63LfDmNb3MQ8mw9MtZ2To9bEA2M71kZUUGq5tiJxcqj9").unwrap(),
    );
    map.insert(
        address!("0xf3BC9c7DaA0aeb694c95d65c2FE86137695F2a59"),
        Pubkey::from_str("ED5nyyWEzpPPiWimP8vYm7sD7TD3LAt3Q3gRTWHzPJBY").unwrap(),
    );
    map.insert(
        address!("0x75939e0A1Eb2321DBaCcDB2E637DdBa29098Eb16"),
        Pubkey::from_str("CzLSujWBLFsSjncfkh59rUFqvafWcY5tzedWJSuypump").unwrap(),
    );
    map.insert(
        address!("0xDD2DDf33d0936A2A4E8316DFc89F538fCc1fA5b1"),
        Pubkey::from_str("ukHH6c7mMyiWCf1b9pnWe25TSpkDDt3H5pQZgZ74J82").unwrap(),
    );
    map.insert(
        address!("0xfFeBa30f39FaA3601911090d5ce7388D719109c9"),
        Pubkey::from_str("A8C3xuqscfmyLrte3VmTqrAq8kgMASius9AFNANwpump").unwrap(),
    );
    map.insert(
        address!("0x9BcbE99c5de789156Aa30eE47C0447BEac2a3B4c"),
        Pubkey::from_str("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263").unwrap(),
    );
    map.insert(
        address!("0x735958915df64598461A5415Ad17EB9a3f98f5ac"),
        Pubkey::from_str("Dz9mQ9NzkBcCsuGPFJ3r1bS4wgqKMHBPiVuniW8Mbonk").unwrap(),
    );

    map
});

pub static TOKEN_TO_OFT: LazyLock<HashMap<Address, Address>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(
        address!("0x420fca0121dc28039145009570975747295f2329"),
        address!("0x1fb8432d5e243986DF8AB85aEa1DfF5030ac54b6"),
    );
    map.insert(
        address!("0xba2ae424d960c26247dd6c32edc70b295c744c43"),
        address!("0x2943191025ae254A7546BdDC57e6173826e4dBc7"),
    );
    map.insert(
        address!("0xc748673057861a797275cd8a068abb95a902e8de"),
        address!("0xc847511c793202C4ef86aE006d81bACeeEd8Ff5C"),
    );
    map.insert(
        address!("0xfb5b838b6cfeedc2873ab27866079ac55363d37e"),
        address!("0x2f225080c88934B211eF41F1247A88F6a1c30Db6"),
    );
    map.insert(
        address!("0x6894cde390a3f51155ea41ed24a33a4827d3063d"),
        address!("0x7510F51A66cE78c9932056B17C26bbbE9f56336D"),
    );
    map.insert(
        address!("0x6982508145454ce325ddbe47a25d4ec3d2311933"),
        address!("0xfDB933367b116f31b24116e761234E631092b7F2"),
    );
    map.insert(
        address!("0x95ad61b0a150d79219dcf64e1e6cc01f0b64c4ce"),
        address!("0x638BDc76db8aACc2cae4A4302334007Ca300d4C9"),
    );
    map.insert(
        address!("0x812ba41e071c7b7fa4ebcfb62df5f45f6fa853ee"),
        address!("0x1666fe591E2F378091c7A9866d80DA445Fd3A1F3"),
    );
    map.insert(
        address!("0xb131f4a55907b10d1f0a50d8ab8fa09ec342cd74"),
        address!("0x7F3B6f46503a7fe22CbEd587d94B5566c99A3f07"),
    );
    map.insert(
        address!("0x761d38e5ddf6ccf6cf7c55759d5210750b5d60f3"),
        address!("0xC803A27157B69F3EEaFC3ff6AF77b40E9D79274c"),
    );
    map.insert(
        address!("0x5026f006b85729a8b14553fae6af249ad16c9aab"),
        address!("0xAEa066b947D2201f60a62537d030aD3dC5cd324D"),
    );
    map.insert(
        address!("0xA35923162C49cF95e6BF26623385eb431ad920D3"),
        address!("0xe7AEA12D012138d4FA937962E3e27677e0088603"),
    );

    map
});
