use rocket::serde::{json::Json, uuid::Uuid, Deserialize, Serialize};
use rocket::Route;
use rocket_db_pools::{sqlx, Connection};

use sqlx::types::Uuid as SqlxUuid;

use crate::db::Db;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
struct Photo {
    id: String,
    cinema_id: String,
    link: String,
}

#[get("/<id>")]
async fn get_one(mut db: Connection<Db>, id: Uuid) -> Option<Json<Photo>> {
    let id = match SqlxUuid::parse_str(&id.to_string()) {
        Ok(id) => id,
        Err(_) => return None,
    };

    sqlx::query!(
        "SELECT id, cinema_id, link FROM dashboard.photo WHERE id = $1",
        id
    )
    .fetch_one(&mut *db)
    .await
    .map(|row| {
        Json(Photo {
            id: row.id.to_string(),
            cinema_id: row
                .cinema_id
                .expect("funny cinema_id is FK but RUST says in may be None")
                .to_string(),
            link: row.link.expect("rightly"),
        })
    })
    .ok()
}

pub fn routes() -> Vec<Route> {
    routes![get_one]
}
