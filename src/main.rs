use dotenv::dotenv;

use yaml_rust::YamlLoader;
use std::io::Read;

mod db;
mod photo;
mod hashing;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    dotenv().ok();


    hashing::hashing_pswd();

    let mut file = std::fs::File::open("config.example.yaml").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    let d = YamlLoader::load_from_str(&contents).unwrap();
    println!("Read YAML string: {:?}", d);

    rocket::build()
        .attach(db::stage())
        .mount("/photo", photo::routes())
}
