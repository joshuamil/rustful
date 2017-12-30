#[macro_use] extern crate nickel;
extern crate hyper;
extern crate toml;
#[macro_use]
extern crate serde_derive;

use nickel::{Nickel, MediaType};

// Enable local modules
mod logger;
mod cors;
mod routes;
mod conf;

fn main() {

    // Use config values
    let address = conf::parse_config();

    // Create new server instance
    let mut server = Nickel::new();

    // Enable middlewares
    server.utilize(cors::enable_cors);
    server.utilize(logger::Logger);
    server.utilize(routes::explicit_router());

    // Enable default router
    server.utilize(router! {
        get "/" => |_request, mut response| {
            response.set(MediaType::Json);
            r#"{ "status": "200", "message": "Server is running" }"#
        }
    });

    // Listen on the configured port
    server.listen(address).unwrap();
}
