# Evm Gas Price Calculator

**⚠️ Warning: This project is experimental and provided as-is. Use it at your own risk, not even sure it's correct.**

This is a Rust program that fetches the current gas price and block number from an Ethereum node or L2s using JSON-RPC calls. It then calculates the priority fees and maximum fees based on the fetched data for multiple Ethereum chains.

## Prerequisites

- Rust programming language (https://www.rust-lang.org/tools/install)
- Cargo package manager (included with Rust)

## Installation

1. Clone the repository: ```git clone https://github.com/malik672/gas-estimator.git```
2. Navigate to the project directory: ```cd src ```
3. Build the project: ```cargo build```

## Usage

1. Run the program with the Ethereum RPC URL as a command-line argument: ```cargo run <ethereum_rpc_url>```
   Replace `<ethereum_rpc_url>` with the actual URL of the Ethereum node you want to fetch data from.

   For example: ```cargo run https://rpc.example.com```


   The program will fetch the gas price, block number, and block info from the specified Ethereum node and calculate the priority fees and maximum fees. The results will be displayed in the console.

   You can run the program with different Ethereum RPC URLs to calculate gas prices for multiple Ethereum chains.

## Note

This project is an example of a project built while learning Rust. It may contain bugs or limitations. Please use it for educational purposes and exercise caution when using it in production environments.

Most importantly calculation of priority fee may not be correct, if you have the solution to this, create an issue

## License

This project is licensed under the [MIT License](LICENSE).
