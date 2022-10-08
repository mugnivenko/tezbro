use dotenv::dotenv;

mod db;
mod photo;
mod views;
mod password_encryption;
mod jwt;
mod role;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(db::stage())
        .mount("/photo", photo::routes())
        .mount("/", views::authentication::routes())
}
