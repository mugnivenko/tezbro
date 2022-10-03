use std::sync::{Arc, Mutex};

use rocket::{form::Form, http::Status, response::Responder, serde::Serialize, Route, State, request::FromRequest};
use rocket_db_pools::{rocket::serde::json::Json, Connection};

use crate::{db::Db, password_encryption::encrypt_password, keys::Keys, jwt::Token, roles::Role};

#[derive(FromForm)]
struct Credentials {
    login: String,
    password: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Tokens {
    access: String,
    refresh: String,
}

#[derive(Serialize)]
#[serde(
    crate = "rocket::serde",
    rename_all = "snake_case",
    tag = "type",
    content = "details"
)]
enum Error {
    UserNotFound,
    BadPassword,
    DatabaseConnectionError,
}

struct Response {
    status: Status,
    body: Result<Json<Tokens>, Json<Error>>,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Response {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        rocket::Response::build_from(self.body.respond_to(request).unwrap())
            .status(self.status)
            .ok()
    }
}

#[post("/authenticate", data = "<credentials>")]
async fn authenticate(mut db: Connection<Db>, keys: State<Arc<Mutex<Keys>>>, credentials: Form<Credentials>) -> Response {
    if let Ok(response) = sqlx::query!(
        "SELECT password, role::text, user_id
        FROM dashboard.user
        LEFT JOIN dashboard.password ON user_id = dashboard.user.id
        WHERE login = $1",
        credentials.login,
    )
    .fetch_optional(&mut *db)
    .await
    {
        if let Some(row) = response {
            let hashed_password = row.password.unwrap();
            if &hashed_password.as_bytes() == &encrypt_password(credentials.password.as_bytes()) {
                Response {
                    status: Status::Ok,
                    body: Ok(Json(Tokens { access: todo!(), refresh: todo!() })),
                }
            } else {
                Response {
                    status: Status::BadRequest,
                    body: Err(Json(Error::BadPassword)),
                }
            }
        } else {
            Response {
                status: Status::BadRequest,
                body: Err(Json(Error::UserNotFound)),
            }
        }
    } else {
        Response {
            status: Status::InternalServerError,
            body: Err(Json(Error::DatabaseConnectionError)),
        }
    }
}

#[derive(FromForm)]
struct RefreshToken {
    token: String,
}

#[post("/refresh", data = "<refresh>")]
async fn refresh(refresh_token: Form<RefreshToken>) -> Tokens {

}

pub fn routes() -> Vec<Route> {
    routes![authenticate]
}
