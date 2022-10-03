use dotenv::dotenv;
use keys::Keys;

mod authentication;
mod db;
mod jwt;
mod keys;
mod password_encryption;
mod photo;
mod roles;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(db::stage())
        .manage(Keys::new(
            std::env::var("ADMIN_PUBLIC_KEY").unwrap().as_bytes().into(),
            std::env::var("CASHIER_PUBLIC_KEY").unwrap().as_bytes().into(),
        ))
        .mount("/photo", photo::routes())
}
