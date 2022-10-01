use dotenv::dotenv;

mod db;
mod photo;
mod hashing;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    dotenv().ok();


    hashing::hashing_pswd();

    rocket::build()
        .attach(db::stage())
        .mount("/photo", photo::routes())
}
