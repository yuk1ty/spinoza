#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct MarketValue<'a> {
    id: u32,
    ticker: &'a str,
    quote: f32,
    high: f32,
    low: f32,
    timestamp: u32,
}

impl<'a> MarketValue<'a> {
    pub fn new(
        id: u32,
        ticker: &'a str,
        quote: f32,
        high: f32,
        low: f32,
        timestamp: u32,
    ) -> MarketValue {
        MarketValue {
            id,
            ticker,
            quote,
            high,
            low,
            timestamp,
        }
    }
}
