use crate::WSClient;
use std::collections::HashMap;

use super::ws_client_internal::WSClientInternal;

const SPOT_WEBSOCKET_URL: &str = "wss://api.huobi.pro/ws";
// const FUTURES_WEBSOCKET_URL: &str = "wss://www.hbdm.com/ws";
// const COIN_SWAP_WEBSOCKET_URL: &str = "wss://api.hbdm.com/swap-ws";
// const USDT_SWAP_WEBSOCKET_URL: &str = "wss://api.hbdm.com/linear-swap-ws";
// const OPTION_WEBSOCKET_URL: &str = "wss://api.hbdm.com/option-ws";
const FUTURES_WEBSOCKET_URL: &str = "wss://futures.huobi.com/ws";
const COIN_SWAP_WEBSOCKET_URL: &str = "wss://futures.huobi.com/swap-ws";
const USDT_SWAP_WEBSOCKET_URL: &str = "wss://futures.huobi.com/linear-swap-ws";
const OPTION_WEBSOCKET_URL: &str = "wss://futures.huobi.com/option-ws";

/// The WebSocket client for Huobi Spot market(<https://huobiapi.github.io/docs/spot/v1/en/>).
pub struct HuobiSpotWSClient {
    client: WSClientInternal,
}

/// The WebSocket client for Huobi Futures market(<https://huobiapi.github.io/docs/dm/v1/en/>).
pub struct HuobiFuturesWSClient {
    client: WSClientInternal,
}

/// The WebSocket client for Huobi Coin Swap market(<https://huobiapi.github.io/docs/coin_margined_swap/v1/en/>).
pub struct HuobiCoinSwapWSClient {
    client: WSClientInternal,
}

/// The WebSocket client for Huobi USDT Swap market(<https://huobiapi.github.io/docs/usdt_swap/v1/en/>).
pub struct HuobiUsdtSwapWSClient {
    client: WSClientInternal,
}

/// The WebSocket client for Huobi Option market(<https://huobiapi.github.io/docs/option/v1/en/>).
pub struct HuobiOptionWSClient {
    client: WSClientInternal,
}

fn serialize_command(channels: &[String], subscribe: bool) -> Vec<String> {
    channels
        .iter()
        .map(|ch| {
            let mut command = HashMap::new();
            command.insert(if subscribe { "sub" } else { "unsub" }, ch.as_str());
            command.insert("id", "crypto-ws-client");
            command
        })
        .map(|m| serde_json::to_string(&m).unwrap())
        .collect::<Vec<String>>()
}

define_client!(HuobiSpotWSClient, SPOT_WEBSOCKET_URL, serialize_command);
define_client!(
    HuobiFuturesWSClient,
    FUTURES_WEBSOCKET_URL,
    serialize_command
);
define_client!(
    HuobiCoinSwapWSClient,
    COIN_SWAP_WEBSOCKET_URL,
    serialize_command
);
define_client!(
    HuobiUsdtSwapWSClient,
    USDT_SWAP_WEBSOCKET_URL,
    serialize_command
);
define_client!(HuobiOptionWSClient, OPTION_WEBSOCKET_URL, serialize_command);
