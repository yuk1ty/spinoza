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
        Ok(Response::with((status::Ok, "Market Value")))
    }

    fn get_positions(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Get Positions")))
    }

    router
}
