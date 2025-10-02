mod fetch_data;
mod variables;
mod write_data;
use std::{env, time};

use crate::{
    IJooceVoting::IJooceVotingInstance,
    fetch_data::{decode_asset_ids, get_ticker, get_weight},
    variables::{
        CHAIN_ID_TO_URL, INACTIVE_ASSETS, JOOCE_INT_WEIGHT, MIN_RELATIVE_WEIGHT, SCALE,
        VOTING_CONTRACT_ADDRESS,
    },
    write_data::{print_hashmap, write_to_google_sheet},
};
use alloy::{
    primitives::{Address, U256, U512, address},
    providers::{DynProvider, Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
    sol,
};
use dotenv::dotenv;

use op_alloy_network::Optimism;

#[derive(Debug, Clone)]
pub struct AssetData {
    id: U256,
    token_addr: Address,
    oft_address: Address,
    symbol: Option<String>,
    chain_id: U256,
    relative_weight: Option<f64>,
    actual_weight: Option<f64>,
    converted_weight: Option<u16>,
}

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IJooceVoting,
    "abi/JooceVoting.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IErc20,
    "abi/Erc20.json"
);

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let time = time::Instant::now();
    let args: Vec<String> = env::args().collect();
    dotenv().ok();
    let pk: PrivateKeySigner = std::env::var("PRIVATE_KEY")
        .expect("Private key missing")
        .parse()?;
    let key = op_alloy_network::EthereumWallet::new(pk);
    let provider = ProviderBuilder::new_with_network::<Optimism>()
        .wallet(key)
        .connect_http(CHAIN_ID_TO_URL.get(&U256::from(8453)).unwrap().parse()?)
        .erased();
    let contract: IJooceVotingInstance<DynProvider<Optimism>, Optimism> =
        IJooceVoting::new(VOTING_CONTRACT_ADDRESS.parse()?, provider.clone());
    let asset_ids: Vec<U256> = contract.assets().call().await?;
    if let Some(val) = args.get(1)
        && val.to_lowercase() == "update"
    {
        update_relative_weight(&contract, &asset_ids).await;
    }
    let mut decoded_data = decode_asset_ids(&asset_ids);
    let jooce = AssetData {
        id: U256::default(),
        symbol: Some("JOOCE".to_owned()),
        token_addr: address!("0x100CE3E3391C00B6A52911313A4Ea8D23c8a38D8"),
        oft_address: address!("0x100CE3E3391C00B6A52911313A4Ea8D23c8a38D8"),
        chain_id: U256::from(8453),
        actual_weight: Some(0.02),
        converted_weight: Some(JOOCE_INT_WEIGHT),
        relative_weight: None,
    };
    let total_weight = contract.weightsSum();

    let (weights, symbols, total_weight) = tokio::join!(
        get_weight(provider, &contract, &decoded_data),
        get_ticker(&decoded_data),
        total_weight.call()
    );
    let total_weight = total_weight.unwrap();
    for (i, asset) in decoded_data.iter_mut().enumerate() {
        asset.relative_weight = Some(u256_division(&weights[i], &total_weight));
        asset.symbol = Some(symbols[i].clone());
    }
    println!("{:?}", decoded_data);
    calculate_actual_weights(&mut decoded_data);
    decoded_data.push(jooce);

    decoded_data.sort_unstable_by(|a, b| {
        b.converted_weight
            .unwrap()
            .cmp(&a.converted_weight.unwrap())
    });

    write_to_google_sheet(&decoded_data).await;
    print_hashmap(&decoded_data);
    println!("{:?}", time.elapsed());
    Ok(())
}

async fn update_relative_weight(
    contract: &IJooceVotingInstance<DynProvider<Optimism>, Optimism>,
    ids: &[U256],
) {
    for id in ids.iter() {
        println!("{}", id);
        let tx = contract.checkpointAsset(*id).send().await;
        match tx {
            Ok(val) => {
                let receipt = val.get_receipt().await;
                println!("{:?}", receipt.unwrap())
            }
            Err(val) => {
                println!("Error with tx - {}", val);
            }
        }
    }
}

fn calculate_actual_weights(asset_data: &mut Vec<AssetData>) {
    asset_data.retain(|asset| {
        !INACTIVE_ASSETS.contains(&asset.token_addr)
            && asset.relative_weight.unwrap() >= MIN_RELATIVE_WEIGHT
    });

    let weight_sum = asset_data
        .iter()
        .fold(0., |acc, x| acc + x.relative_weight.unwrap());

    asset_data.iter_mut().for_each(|asset| {
        let relative_weight = asset.relative_weight.unwrap();
        let actual_weight = (relative_weight / weight_sum) * 0.98;
        let converted_weight = (u16::MAX as f64 * actual_weight) as u16;
        asset.actual_weight.replace(actual_weight);
        asset.converted_weight.replace(converted_weight);
    });
    let adjusted_sum = asset_data
        .iter()
        .fold(0, |acc, x| acc + x.converted_weight.unwrap());

    let remainder = u16::MAX - JOOCE_INT_WEIGHT - adjusted_sum;
    let base = remainder / asset_data.len() as u16;
    let extra = remainder % asset_data.len() as u16;

    asset_data.iter_mut().for_each(|asset| {
        asset
            .converted_weight
            .replace(asset.converted_weight.unwrap() + base);
    });

    asset_data
        .iter_mut()
        .take(extra as usize)
        .for_each(|asset| {
            asset
                .converted_weight
                .replace(asset.converted_weight.unwrap() + 1);
        });
}

fn u256_division(numerator: &U256, denominator: &U256) -> f64 {
    let numerator = numerator.to::<U512>() * U512::from(SCALE);
    println!("{}", numerator);
    let quotient = numerator / denominator.to::<U512>();
    println!("{}", quotient);
    quotient.to::<u128>() as f64 / SCALE as f64
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn check_conversion() {
        let num_one = U256::from(1000);
        let num_two = U256::from(1000);
        let result = u256_division(&num_one, &num_two);
    }
}
