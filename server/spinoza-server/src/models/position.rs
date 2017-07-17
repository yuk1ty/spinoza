use models::currency::*;
use models::market_value::*;

#[derive(PartialEq, Clone)]
pub struct Position<'a> {
    id: u32,
    currency: &'a Currency<'a>, // TODO need it to be ref?
    amount: i32,
    market_value: &'a MarketValue<'a>,
    acquisition_cost: f32,
}

impl<'a> Position<'a> {
    pub fn new(
        id: u32,
        currency: &'a Currency,
        amount: i32,
        market_value: &'a MarketValue,
        acquisition_cost: f32,
    ) -> Position<'a> {
        Position {
            id,
            currency,
            amount,
            market_value,
            acquisition_cost,
        }
    }
}
