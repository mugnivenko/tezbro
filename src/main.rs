use dotenv::dotenv;

mod db;
mod photo;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(db::stage())
        .mount("/photo", photo::routes())
}
