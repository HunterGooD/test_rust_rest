use std::collections::HashMap;
use std::iter;
use std::sync::Arc;

use rocket::futures::future;
use rocket::http::ContentType;
use rocket::serde::json::{json, Value};
use rocket::tokio::{self, sync::Mutex};
use rocket::Data;
use rocket_multipart_form_data::{ mime,
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

use crate::internal::image as image_clasify;

const MAX_FILES: u8 = 10;

#[post("/upload", data = "<data>")]
pub async fn uploads_file(content_type: &ContentType, data: Data<'_>) -> Value {
    // let files_field = vec![MultipartFormDataField::file("file")].repeat(MAX_FILES as usize);
    let files_field = iter::repeat(MultipartFormDataField::file("file").content_type(Some(mime::IMAGE_STAR)))
        .take(MAX_FILES as usize)
        .collect();
    let options = MultipartFormDataOptions {
        allowed_fields: files_field,
        ..MultipartFormDataOptions::default()
    };

    let multipart_form_data = MultipartFormData::parse(content_type, data, options)
        .await
        .unwrap();

    // println!("{:?}", multipart_form_data);

    let files = match multipart_form_data.files.get("file") {
        None => return json!({"error": "No file upload"}),
        Some(v) => v,
    };
    
    let clasify_response: Arc<Mutex<HashMap<String, String>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let mut wait_files: Vec<_> = vec![];
    for _file in files {
        let clone = Arc::clone(&clasify_response);
        let path = String::from(_file.path.to_str().unwrap());
        let file_name = _file.file_name.as_ref().unwrap();
        let file_name = String::from(file_name);

        let worker = tokio::spawn(async move {
            image_clasify::process_image(&clone, &file_name[..], &path[..])
                .await
                .expect("Error proccess image");
        });

        wait_files.push(worker);
    }

    future::join_all(wait_files).await;

    let clasify_response = Arc::try_unwrap(clasify_response).unwrap().into_inner();

    json!(clasify_response)
    // data.file.persist_to(filepath).await.expect("file upload error！");
}
