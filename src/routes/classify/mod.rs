mod model;

use model::MyResponse;
use rocket::serde::json::{Json};

#[post("/")]
pub fn classify() -> Json<MyResponse> {
    Json(MyResponse { name: "123".to_string()})
}
