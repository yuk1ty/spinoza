extern crate spinoza_server;

extern crate router;
extern crate iron;

use iron::Iron;

use spinoza_server::handlers::*;

fn main() {
    Iron::new(prepare_router()).http("localhost:9999").unwrap();
}
