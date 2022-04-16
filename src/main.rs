#[macro_use]
extern crate rocket;

mod internal;
mod routes;

use crate::routes::{classify, greeting, not_found, uploads};

// default run rocket
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/hello", routes![greeting::hello])
        .mount("/classify", routes![classify::classify])
        .mount("/file", routes![uploads::uploads_file])
        .register("/", catchers![not_found::not_found])
}
// others run app rocket

// #1
// #[rocket::main]
// async fn main() {
//     let result = rocket::build()
//     .mount("/", routes![hello])
//     .launch().await;

//     // this is reachable only after `Shutdown::notify()` or `Ctrl+C`.
//     println!("Rocket: deorbit.");
// }

// #2 with catch error
// fn rocket() -> Rocket<Build> {
//     rocket::build()
//         // .mount("/", routes![hello, hello]) // uncomment this to get an error
//         // .mount("/", routes![unmanaged]) // uncomment this to get a sentinel error
//         .mount("/", routes![hello, forced_error])
//         .register("/", catchers![general_not_found, default_catcher])
//         .register("/hello", catchers![hello_not_found])
//         .register("/hello/Sergio", catchers![sergio_error]) // используется для перехвата роута и ошибок
// }

// #[rocket::main]
// async fn main() {
//     if let Err(e) = rocket().launch().await {
//         println!("Whoops! Rocket didn't launch!");
//         // We drop the error to get a Rocket-formatted panic.
//         drop(e);
//     };
// }
