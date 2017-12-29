extern crate nickel;
use nickel::{Nickel, HttpRouter};

pub fn explicit_router() -> nickel::Router {

    let mut router = Nickel::router();

    router.get("/v*/**", middleware! {
        "This matches the route pattern"
    });

    router
}
