use crate::controllers::eth::ScannerConfig;

pub fn get_supported_networks() -> Vec<ScannerConfig> {
    [
        ScannerConfig {
            chain_name: "Ethereum Sepolia".to_string(),
            prefix: "eth-sepolia".to_string(), // prefix is used to create the route "/eth-sepolia
            chain_symbol: "ETH".to_string(),
            api_url: "https://api-sepolia.etherscan.io/api".to_string(),
        },
        ScannerConfig {
            chain_name: "Ethereum".to_string(),
            prefix: "eth".to_string(), // prefix is used to create the route "/eth
            chain_symbol: "ETH".to_string(),
            api_url: "https://api.etherscan.io/api".to_string(),
        },
        ScannerConfig {
            chain_name: "Polygon".to_string(),
            prefix: "polygon".to_string(), // prefix is used to create the route "/polygon
            chain_symbol: "POLYGON".to_string(),
            api_url: "https://api.polygonscan.com/api".to_string(),
        },
        ScannerConfig {
            chain_name: "Polygon Matic".to_string(),
            prefix: "polygon-matic".to_string(), // prefix is used to create the route "/polygon-matic
            chain_symbol: "POLYGON".to_string(),
            api_url: "https://api-testnet.polygonscan.com/api".to_string(),
        },
        ScannerConfig {
            chain_name: "Binance Smart Chain".to_string(),
            prefix: "bsc".to_string(), // prefix is used to create the route "/bsc
            chain_symbol: "BNB".to_string(),
            api_url: "https://api.bscscan.com/api".to_string(),
        },
    ]
    .to_vec()
}
