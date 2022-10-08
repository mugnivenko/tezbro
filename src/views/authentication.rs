use std::borrow::Borrow;

use rocket::{
    form::Form,
    http::Status,
    serde::{json::Json, Serialize},
    FromForm, Route,
};
use rocket_db_pools::Connection;

use crate::{
    db::Db,
    jwt::{make_token, TokenType, self},
    password_encryption::encrypt,
    role::Role,
};

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

#[post("/authenticate", data = "<credentials>")]
async fn authenticate(
    mut db: Connection<Db>,
    credentials: Form<Credentials>,
) -> Result<Json<Tokens>, Status> {
    if let Ok(row) = sqlx::query!(
        "SELECT password, role::text, user_id
        FROM dashboard.user
        LEFT JOIN dashboard.password ON user_id = dashboard.user.id
        WHERE login = $1",
        credentials.login,
    )
    .fetch_optional(&mut *db)
    .await
    {
        if let Some(row) = row {
            let hashed_password = row.password.unwrap();
            let role = row.role.unwrap();
            let user_id = row.user_id.unwrap().as_u128();
            if encrypt(credentials.password.as_bytes()).borrow() == hashed_password.as_bytes() {
                Ok(Json(Tokens {
                    access: make_token(user_id, &Role::from_str(&role).unwrap(), &TokenType::Access),
                    refresh: make_token(user_id, &Role::from_str(&role).unwrap(), &TokenType::Refresh),
                }))
            } else {
                Err(Status::BadRequest)
            }
        } else {
            Err(Status::BadRequest)
        }
    } else {
        Err(Status::InternalServerError)
    }
}

#[derive(FromForm)]
pub struct RefreshToken {
    refresh_token: String,
}

#[allow(clippy::unused_async)]
#[post("/refresh", data = "<refresh_token>")]
async fn refresh(refresh_token: Form<RefreshToken>) -> Result<Json<Tokens>, Status> {
    if let Ok(claims) = jwt::decode(&refresh_token.refresh_token) {
        let user_id = claims.sub;
        let role = &claims.role;
        Ok(Json(Tokens {
            access: make_token(user_id, &Role::from_str(role).unwrap(), &TokenType::Access),
            refresh: make_token(user_id, &Role::from_str(role).unwrap(), &TokenType::Refresh),
        }))
    } else {
        Err(Status::BadRequest)
    }
}

pub fn routes() -> Vec<Route> {
    routes![authenticate, refresh]
}
