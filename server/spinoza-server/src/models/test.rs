use models::currency::*;
use models::market_value::*;
use models::position::*;
use models::enums::*;

#[test]
fn eq_currency_test() {
    let this: Currency = Currency::new(1 as u32, "BitCoin", Exchange::CoinChecker);
    let other: Currency = Currency::new(1 as u32, "BitCoin", Exchange::CoinChecker);
    assert!(this.eq(&other));

    let another: Currency = Currency::new(2 as u32, "BitCoin", Exchange::BitFlyer);
    assert!(!this.eq(&another));
}

#[test]
fn eq_market_value_test() {
    let this = MarketValue::new(1 as u32, "BitCoin", 120.0, 90.0, 14412234);
    let other = MarketValue::new(1 as u32, "BitCoin", 120.0, 90.0, 14412234);

    assert!(this.eq(&other));

    let another = MarketValue::new(2 as u32, "BitCoin", 120.0, 90.0, 14412234);
    assert!(!this.eq(&another));
}

#[test]
fn eq_position_test() {
    let currency = Currency::new(1 as u32, "BitCoin", Exchange::CoinChecker);
    let market_value = MarketValue::new(1 as u32, "BitCoin", 120.0, 90.0, 14412234);
    let this = Position::new(1 as u32, &currency, 1, &market_value, 100.0);
    let other = Position::new(1 as u32, &currency, 1, &market_value, 100.0);
    assert!(this.eq(&other));

    // change market_values
    let another = Position::new(2 as u32, &currency, 1, &market_value, 101.0);
    assert!(!this.eq(&another));

    // change currency
    let other_currency = Currency::new(2 as u32, "BitCoin", Exchange::CoinChecker);
    let _another = Position::new(1 as u32, &other_currency, 1, &market_value, 100.0);
    assert!(!this.eq(&_another));
}

#[test]
fn enum_from_code_test() {
    let exchange = Exchange::from_code("BIT_FLYER");
    assert!(exchange.unwrap() == Exchange::BitFlyer);
}

#[test]
fn enum_to_code_test() {
    let exchange = Exchange::BitFlyer;
    assert!(exchange.to_code().eq(&String::from("BIT_FLYER")));
}
