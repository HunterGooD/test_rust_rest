use rocket::serde::json::{Value, json};

#[catch(404)]
pub fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}