use ethers::prelude::*;
// use ethers::core::types::Block;
use ethers::types::U256;
use reqwest::Client;
use serde_json;
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug)]
enum EthRpcMethod {
    EthGasPrice,
    EthBlockNumber,
    EthGetBlockByNumber,
}

impl EthRpcMethod {
    fn as_str(&self) -> &str {
        match self {
            EthRpcMethod::EthGasPrice => "eth_gasPrice",
            EthRpcMethod::EthBlockNumber => "eth_blockNumber",
            EthRpcMethod::EthGetBlockByNumber => "eth_getBlockByNumber",
        }
    }
}

#[derive(serde::Deserialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    result: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ethereum_rpc_url = "https://rpc.mevblocker.io"; // Replace with your Ethereum node's RPC URL

    // Fetch gas price
    let gas_price_wei = fetch_ethereum_data(&EthRpcMethod::EthGasPrice, ethereum_rpc_url).await?;
    println!("Current Gas Price (in Gwei): {}", gas_price_wei);

    //Fetch block Number
    let gas_block_number =
        fetch_ethereum_data(&EthRpcMethod::EthBlockNumber, ethereum_rpc_url).await?;
    println!("Block Number: {}", gas_block_number);

    // Fetch block number
    // let block_number_hex =
    //     fetch_ethereum_data(&EthRpcMethod::EthBlockNumber, ethereum_rpc_url).await?;
    // let get_block =
    //     fetch_ethereum_data(&EthRpcMethod::EthGetBlockByNumber, ethereum_rpc_url).await?;

    let fetch_block = fetch_block_by_number(
        &gas_block_number,
        &EthRpcMethod::EthGetBlockByNumber,
        ethereum_rpc_url,
    )
    .await?;
    println!("Current Gas Price (in Gwei): {}", fetch_block);

    Ok(())
}

async fn fetch_ethereum_data(
    method: &EthRpcMethod,
    rpc_url: &str,
) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let params = serde_json::Value::Array(vec![]);

    let body = json!({
        "jsonrpc": "2.0",
        "method": method.as_str(),
        "params": params,
        "id": 1, // You can use any unique ID
    });

    let response = client.post(rpc_url).json(&body).send().await?;

    let json: JsonRpcResponse = response.json().await?;
    Ok(json.result)
}

async fn fetch_block_by_number(
    number: &str,
    method: &EthRpcMethod,
    rpc_url: &str,
) -> Result<String, reqwest::Error> {
    let client = Client::new();

    let params = serde_json::Value::Array(vec![json!(number), json!(true)]);

    let body = json!({
        "jsonrpc": "2.0",
        "method": method.as_str(),
        "params": params,
        "id": 1, // You can use any unique ID
    });

    let response = client.post(rpc_url).json(&body).send().await?;

    let json: HashMap<String, serde_json::Value> = response.json().await?;

    // Iterating over the key-value pairs and printing them
    let red = json.get("result").cloned().unwrap();
    let map_red = red.as_object().unwrap();
    let base_fee_per_gas = map_red.get("baseFeePerGas").cloned().unwrap();
    Ok(base_fee_per_gas.to_string())
}

fn convert_wei_to_gwei(wei_value: &str) -> f64 {
    let wei_value = wei_value.parse::<f64>().unwrap();
    wei_value / 1_000_000_000.0
}

fn parse_hex_to_u64(hex_value: &str) -> u64 {
    u64::from_str_radix(&hex_value[2..], 16).unwrap()
}
