use rocket::fs::TempFile;

#[post("/upload", data = "<file>")]
pub async fn uploads_file(mut file: TempFile<'_>)  {
    // file.persist_to("").await;
    match file.name() {
        Some(name) => println!("File name {}", name),
        None => println!("Not file upload"),
    }
} 