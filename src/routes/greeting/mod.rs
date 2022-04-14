#[get("/<name>")]
pub fn hello(name: &str) -> String {
    let mut a = "Hello, ".to_string();
    a.push_str(name);
    a
}
