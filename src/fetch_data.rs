use crate::{
    AssetData, IErc20,
    IJooceVoting::{self, IJooceVotingInstance},
    variables::{ADDR_TO_SOL_MINT_ADDR, CHAIN_ID_TO_URL, SOLANA_CHAIN_ID},
};
use alloy::{
    primitives::{Address, U256, address},
    providers::{DynProvider, Dynamic, MulticallBuilder, Provider, ProviderBuilder},
};
use op_alloy_network::{Ethereum, Optimism};
use solana_client::rpc_client;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;

pub async fn get_relative_weight(
    provider: DynProvider<Optimism>,
    contract: &IJooceVotingInstance<DynProvider<Optimism>, Optimism>,
    asset_data: &[AssetData],
) -> Vec<U256> {
    let mut multicall: MulticallBuilder<
        Dynamic<IJooceVoting::relativeWeightCall>,
        &DynProvider<Optimism>,
        Optimism,
    > = provider.multicall().dynamic();

    for asset in asset_data.iter() {
        multicall = multicall.add_dynamic(contract.relativeWeight(asset.id));
    }
    let result = multicall.aggregate().await.unwrap();

    assert_eq!(&result.len(), &asset_data.len(), "Unequal arrays");

    result
}

pub type ProviderMap = HashMap<
    U256,
    Option<(
        Vec<usize>,
        DynProvider<Ethereum>,
        MulticallBuilder<Dynamic<IErc20::symbolCall>, DynProvider, Ethereum>,
    )>,
>;

pub async fn get_ticker(asset_data: &[AssetData]) -> Vec<String> {
    let mut provider_map: ProviderMap = HashMap::new();
    let mut solana_tokens: Vec<Pubkey> = Vec::new();
    let mut solana_indices: Vec<usize> = Vec::new();
    let mut symbols: Vec<String> = vec![String::new(); asset_data.len()];

    for (i, asset) in asset_data.iter().enumerate() {
        if asset.chain_id != SOLANA_CHAIN_ID {
            provider_map
                .entry(asset.chain_id)
                .and_modify(|e| {
                    let (mut indices, provider, multicall) = e.take().unwrap();
                    indices.push(i);
                    let contract = IErc20::new(asset.token_addr, provider.clone());
                    let multicall = multicall.add_dynamic(contract.symbol());
                    *e = Some((indices, provider, multicall));
                })
                .or_insert({
                    let provider = ProviderBuilder::new()
                        .connect_http(
                            CHAIN_ID_TO_URL
                                .get(&asset.chain_id)
                                .unwrap()
                                .parse()
                                .unwrap(),
                        )
                        .erased();
                    let multicall_provider = provider.multicall().with_cloned_provider().dynamic();
                    let contract = IErc20::new(asset.token_addr, provider.clone());
                    let mut index_vec = Vec::with_capacity(asset_data.len());
                    index_vec.push(i);
                    Some((
                        index_vec,
                        provider,
                        multicall_provider.add_dynamic(contract.symbol()),
                    ))
                });
        } else {
            let mint_pubkey = ADDR_TO_SOL_MINT_ADDR.get(&asset.token_addr).unwrap();
            let pda = mpl_token_metadata::accounts::Metadata::find_pda(mint_pubkey).0;
            solana_tokens.push(pda);
            solana_indices.push(i);
        }
    }

    let mut futures_vec = Vec::new();
    let mut index_vec = Vec::new();
    for (_k, v) in provider_map.into_iter() {
        let (indices, _provider, multicall) = v.unwrap();
        let fut = async move { multicall.aggregate().await };
        futures_vec.push(fut);
        index_vec.push(indices);
    }
    let results = futures::future::try_join_all(futures_vec).await.unwrap();
    results
        .into_iter()
        .flatten()
        .zip(index_vec.into_iter().flatten())
        .for_each(|(symbol, index)| {
            symbols[index] = symbol.to_uppercase();
        });

    let sol_provider = rpc_client::RpcClient::new(CHAIN_ID_TO_URL.get(&SOLANA_CHAIN_ID).unwrap());
    let sol_metadata_account = sol_provider.get_multiple_accounts(&solana_tokens).unwrap();
    for (account, idx) in sol_metadata_account.iter().zip(solana_indices) {
        if let Some(val) = account {
            let metadata = mpl_token_metadata::accounts::Metadata::safe_deserialize(&val.data);
            match metadata {
                Ok(val) => {
                    symbols[idx] = val.symbol.trim_end_matches('\0').to_uppercase();
                }
                Err(_) => {
                    println!(
                        "Metadata decoding failed for {:?}",
                        asset_data[idx].token_addr
                    )
                }
            }
        }
    }
    symbols
}

pub fn decode_asset_ids(asset_ids: &[U256]) -> Vec<AssetData> {
    let mut out = Vec::with_capacity(asset_ids.len());
    for x in asset_ids {
        let byte_array = x.to_be_bytes::<32>();
        let token_addr = Address::from_slice(&byte_array[12..]);
        let chain_id = if token_addr == address!("0xa697e272a73744b343528c3bc4702f2565b2f422") {
            SOLANA_CHAIN_ID
        } else {
            U256::from_be_slice(&byte_array[0..12])
        };

        out.push(AssetData {
            id: *x,
            token_addr,
            chain_id,
            relative_weight: None,
            actual_weight: None,
            converted_weight: None,
            symbol: None,
        });
    }
    out
}
