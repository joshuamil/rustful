extern crate hyper;
extern crate nickel;
use nickel::{Nickel, HttpRouter, MediaType};
use nickel::status::StatusCode;

pub fn explicit_router() -> nickel::Router {

    let mut router = Nickel::router();

    router.post("/v1/event", middleware! { |_request, mut response|
        response.set(MediaType::Json)
            .set(StatusCode::Ok);
        r#"{ "status": "200", "message": "Register an event with the publisher" }"#
    });

    router.post("/v1/event/broadcast", middleware! { |_request, mut response|
        response.set(MediaType::Json)
            .set(StatusCode::Ok);
        r#"{ "status": "200", "message": "Broadcast an event to subscribers" }"#
    });

    router.get("/v1/event", middleware! { |_request, mut response|
        response.set(MediaType::Json)
            .set(StatusCode::Ok);
        r#"{ "status": "200", "message": "Retrieve a list of events" }"#
    });

    router.get("/v1/event/**", middleware! { |_request, mut response|
        response.set(MediaType::Json)
            .set(StatusCode::Ok);
        r#"{ "status": "200", "message": "Retrieve a specific event" }"#
    });

    router
}
