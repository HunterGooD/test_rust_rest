use std::{collections::HashMap, vec, io::Cursor};

use rocket::tokio::{self, fs::{self, File}, sync::Mutex, io::{AsyncReadExt, AsyncWriteExt}};

pub mod facedetect;

pub async fn process_image(
    m: &Mutex<HashMap<String, String>>,
    file_name: &str,
    path: &str,
) -> Result<(), image::ImageError> {
    println!("Path: {}, name: {}", path, file_name);

    let mut _file = match File::open(path).await {
        Err(why) => panic!("couldn't open {}: {}", path, why),
        Ok(file) => file,
    };
    let mut buf = vec![];

    _file.read_to_end(&mut buf).await.expect("Error read file");

    let sv_path = String::from("images/") + (file_name);
    println!("{}", sv_path.to_lowercase());
    let mut _file = File::create(sv_path.to_lowercase()).await.unwrap();
    _file.write_all_buf(&mut Cursor::new(&buf)).await.expect("Error write buf to file");
    image::load_from_memory(&buf);
        // let file_image = match image::open(path) {
    //     Ok(f_image) => f_image,
    //     Err(e) => return Err(e),
    // };
    // let sv_path = String::from("/images/") + (file_name);
    // match file_image.save(sv_path) {
    //     Ok(_) => {}
    //     Err(e) => return Err(e),
    // };
    // let mut h_m = m.lock().await;
    // h_m.insert(file_name.to_string(), String::from("sucess"));
    
    Ok(())
}
