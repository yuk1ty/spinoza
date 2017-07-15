use models::enums::Exchange;

#[derive(PartialEq, Hash, Clone)]
pub struct Currency<'a> {
    id: u32,
    ticker: &'a str,
    exchange: Exchange,
}

impl<'a> Currency<'a> {
    pub fn new(id: u32, ticker: &'a str, exchange: Exchange) -> Currency<'a> {
        Currency {
            id,
            ticker,
            exchange,
        }
    }
}
