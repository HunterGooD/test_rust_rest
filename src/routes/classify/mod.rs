mod model;

use rocket::serde::json::Json;

use model::MyResponse;

#[post("/")]
pub fn classify() -> Json<MyResponse> {
    Json(MyResponse {
        name: "123".to_string(),
    })
}
