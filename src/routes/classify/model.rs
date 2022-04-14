use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct MyResponse {
    #[serde(rename = "name")]
    pub name: String,
}
