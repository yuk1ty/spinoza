pub struct Currency<'a> {
    id: u32,
    name: &'a str,
    exchange: &'a str,
}

impl<'a> Currency<'a> {
    fn new(&'a self, id: u32, name: &'a str, exchange: &'a str) -> Currency {
        Currency { id, name, exchange }
    }
}
