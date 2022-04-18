use std::time::Instant;
use std::{collections::HashMap, vec};

use rocket::tokio::{self, fs::File, io::AsyncReadExt, sync::Mutex};

pub mod facedetect;

pub async fn process_image(
    m: &Mutex<HashMap<String, String>>,
    file_name: &str,
    path: &str,
) -> Result<(), image::ImageError> {
    let now = Instant::now();

    println!("Path: {}, name: {}", path, file_name);

    let mut _file = match File::open(path).await {
        Err(why) => panic!("couldn't open {}: {}", path, why),
        Ok(file) => file,
    };

    let mut buf: Vec<u8> = vec![];
    _file.read_to_end(&mut buf).await.expect("Error read file");

    let d_image = image::load_from_memory(&buf).expect("error load buf to image");

    let sv_path = String::from("images/") + (file_name);
    d_image
        .save(sv_path.to_lowercase())
        .expect("Error save image");

    let mut h_m = m.lock().await;
    h_m.insert(file_name.to_string(), String::from("sucess"));

    let elapsed_time = now.elapsed();
    println!(
        "Running save file {} took {} seconds.",
        file_name,
        elapsed_time.as_secs()
    );
    Ok(())
}
