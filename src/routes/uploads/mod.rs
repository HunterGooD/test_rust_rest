mod model;

use model::UploadUpdate;
use rocket::form::Form;

#[post("/upload", data = "<data>")]
pub async fn uploads_file(mut data: Form<UploadUpdate<'_>>) {
    // let file_name = data.file.name().unwrap();
    println!("{:?}", data);
    // data.file.persist_to(filepath).await.expect("file upload error！");
}
