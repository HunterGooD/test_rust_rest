// use rocket::futures::future::join_all; //https://docs.rs/futures/0.3.5/futures/future/fn.join_all.html
use rocket::serde::json::{json, Value};
use rocket::tokio::{
    self, /*, sync::mpsc, task*/
    time::{sleep, Duration},
};

#[get("/<name>")]
pub async fn hello(name: &str) -> Value {
    let v = vec!["".to_string(); 100_000];
    let v: Vec<_> = v
        .into_iter()
        .map(|mut item| {
            tokio::spawn(async {
                sleep(Duration::from_millis(500)).await;
                item = "123".to_string();
                item
            })
        })
        .collect();

    let mut items = vec![];
    for task in v {
        items.push(task.await.unwrap());
    }

    json!({ name: items })
}
