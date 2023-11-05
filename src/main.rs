use std::env;
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
    // let ethereum_rpc_url = "https://rpc.mevblocker.io"; // Replace with your Ethereum node's RPC URL

    let args: Vec<String> = env::args().collect();
    let ethereum_rpc_url = match args.get(1) {
        Some(url) => url,
        None => {
            eprintln!("Please provide the Ethereum RPC URL as a command-line argument");
            return Ok(());
        }
    };

    // Fetch gas price
    let gas_price_wei = fetch_ethereum_data(&EthRpcMethod::EthGasPrice, ethereum_rpc_url).await?;
    println!("Current Gas Price (in Gwei): {}", gas_price_wei);

    //Fetch block Number
    let gas_block_number =
        fetch_ethereum_data(&EthRpcMethod::EthBlockNumber, ethereum_rpc_url).await?;
    println!("Block Number: {}", gas_block_number);

    //fetch block info using block number
    let fetch_block = fetch_block_by_number(
        &gas_block_number,
        &EthRpcMethod::EthGetBlockByNumber,
        ethereum_rpc_url,
    )
    .await?;
    let hex = fetch_block.trim_matches('"');
    let gas_price_wei = hex_to_decimal(&gas_price_wei);
    let priority_fee = calculate_priority_fees(gas_price_wei as f64);
    let max_fee_slow = priority_fee.0 + hex_to_decimal(hex) as f64;
    let max_fee_standard = priority_fee.1 + hex_to_decimal(hex) as f64;
    let max_fee_fast = priority_fee.2 + hex_to_decimal(hex) as f64;
    

    let result = json!({
        "slow": {
            "priority fee": priority_fee.0,
            "max_fee": max_fee_slow
        },
        "standard": {
            "priority fee": priority_fee.1,
            "max_fee": max_fee_standard
        },
        "fast": {
            "priority fee": priority_fee.2,
            "max_fee": max_fee_fast
        },
        "base_fee": hex_to_decimal(hex) as f64,
        "block_number": gas_block_number,
        "Note":"every result is in wwei"
    });
    println!("Result: {:?}", result);

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


fn calculate_priority_fees(standard_gas_price: f64) -> (f64, f64, f64) {
    // Define the percentage differences for slow and fast options
    let slow_percentage = 0.8; // 80% of the standard gas price
    let fast_percentage = 1.2; // 120% of the standard gas price

    // Calculate the slow, standard, and fast priority fees
    let slow_fee = standard_gas_price * slow_percentage;
    let standard_fee = standard_gas_price;
    let fast_fee = standard_gas_price * fast_percentage;

    (slow_fee, standard_fee, fast_fee)
}


fn hex_to_decimal(hex: &str) -> u64 {
    // Remove the optional "0x" prefix from the hex string
    let hex = hex.trim_start_matches("0x");
    // Parse the hex string as a u64 with radix 16
    u64::from_str_radix(hex, 16).unwrap()
}
