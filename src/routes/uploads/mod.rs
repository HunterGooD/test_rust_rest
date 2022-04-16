use std::iter;

use rocket::http::ContentType;
use rocket::serde::json::{json, Value};
use rocket::Data;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

// use crate::internal::image; 

const MAX_FILES: u8 = 10;

#[post("/upload", data = "<data>")]
pub async fn uploads_file(content_type: &ContentType, data: Data<'_>) -> Value {
    // let files_field = vec![MultipartFormDataField::file("file")].repeat(MAX_FILES as usize);
    let files_field = iter::repeat(MultipartFormDataField::file("file"))
        .take(MAX_FILES as usize)
        .collect();
    let options = MultipartFormDataOptions {
        allowed_fields: files_field,
        ..MultipartFormDataOptions::default()
    };

    let multipart_form_data = MultipartFormData::parse(content_type, data, options)
        .await
        .unwrap();

    println!("{:?}", multipart_form_data);

    let wait_files = match multipart_form_data.files.get("file") {
        None => return json!({"error": "Not file upload"}),
        Some(v) => v,
    };
    let mut paths: Vec<String> = vec![];
    for file in wait_files {
        paths.push(file.path.to_str().unwrap().to_string());
    }

    json!({ "paths": paths })
    // data.file.persist_to(filepath).await.expect("file upload error！");
}
