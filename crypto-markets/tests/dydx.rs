use crypto_markets::{fetch_symbols, get_market_types, MarketType};

#[macro_use]
mod utils;

const EXCHANGE_NAME: &str = "dydx";

#[test]
fn fetch_all_symbols() {
    gen_all_symbols!();
}

#[test]
fn fetch_linear_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!symbols.is_empty());

    for symbol in symbols.iter() {
        assert!(symbol.ends_with("-USD"));
    }
}
