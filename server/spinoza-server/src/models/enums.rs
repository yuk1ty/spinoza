#[derive(PartialEq, Hash, Clone)]
pub enum Exchange {
    BitFlyer,
    CoinChecker,
}

pub trait UserType<T> {
    fn from_code(code: &'static str) -> Option<T>;

    fn to_code(&self) -> &'static str;
}

impl UserType<Exchange> for Exchange {
    fn from_code(code: &'static str) -> Option<Exchange> {
        match code {
            "BIT_FLYER" => Some(Exchange::BitFlyer),
            "COIN_CHECKER" => Some(Exchange::CoinChecker),
            _ => None,
        }
    }

    fn to_code(&self) -> &'static str {
        match self {
            &Exchange::BitFlyer => "BIT_FLYER",
            &Exchange::CoinChecker => "COIN_CHECKER",
        }
    }
}
