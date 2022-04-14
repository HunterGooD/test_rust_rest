use rocket::fs::TempFile;

#[derive(Debug, FromForm)]
pub struct UploadUpdate<'r> {
    pub file: TempFile<'r>,
}
