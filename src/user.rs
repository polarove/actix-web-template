use actix_web::{web, HttpResponse};
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
struct Entity {
    id: u32,
    create_time: DateTime<Utc>,
    update_time: DateTime<Local>,
    is_deleted: bool,
}

// write a user struct to impl Entity
#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
struct User<Entity> {
    meta: Entity,
    name: String,
}
impl Display for User<Entity> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "user is {}", self.name)
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().json(get_user()) }))
            .route(web::post().to(|| async { HttpResponse::Ok().body("posddt") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

fn get_user() -> User<Entity> {
    let entity = Entity {
        id: 1,
        create_time: Utc::now(),
        update_time: Local::now(),
        is_deleted: false,
    };
    let user = User {
        meta: entity,
        name: String::from("test"),
    };

    format!("user is {}", user);
    user
}
