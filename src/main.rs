// #[macro_use] allows for macros to be used globally.
#[macro_use] extern crate rocket;
use rocket::{Build, Rocket};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index])
}

#[rocket::main]
async fn main() -> () {
    let _ = rocket().launch().await;
}