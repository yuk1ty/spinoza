extern crate iron;

use iron::{Request, Response, IronResult};
use iron::status;
use router::Router;


pub fn prepare_router() -> Router {
    let mut router = Router::new();
    router.get("/", root, "root");
    router.get(
        "/acquire/marketValue",
        acquire_market_value,
        "acquire_market_value",
    );
    router.get("/get/positions", get_positions, "get_positions");

    /// Routing
    fn root(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello, Spinoza!")))
    }

    fn acquire_market_value(_: &mut Request) -> IronResult<Response> {
        // json sample
        let market_value_json = json!({
            "ticker": "bit coin",
            "quote": 100.0,
            "high": 120.0,
            "low": 100.0,
            "timestamp": 1441234,
        });

        Ok(Response::with((status::Ok, market_value_json.to_string())))
    }

    fn get_positions(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Get Positions")))
    }

    router
}
