use models::currency::*;

pub struct Position<'a> {
    currency: Currency<'a>,
    amount: i32,
    market_value: f32,
    acquisition_cost: f32,
}
