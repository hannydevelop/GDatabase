#![feature(proc_macro_hygiene, decl_macro)]
#![feature(option_result_contains)]

#[macro_use]
mod routes;
use rocket::{launch, routes};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![routes::data::post_data])
}