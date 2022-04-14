#[macro_use]
extern crate rocket;

// fn main() {
//     rocket::custom(config::from_env())
//         .launch();
// }
// #[rocket::main]
// async fn main() {
//     let result = rocket::build()
//     .mount("/", routes![hello])
//     .launch().await;

//     // this is reachable only after `Shutdown::notify()` or `Ctrl+C`.
//     println!("Rocket: deorbit.");
// }

mod routes;
use crate::routes::greeting;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/hello", routes![greeting::hello])
}
