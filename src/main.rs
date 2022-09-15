mod photo;
mod db;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::stage())
        .mount("/", routes![index])
        .mount("/photo", photo::routes())
}
