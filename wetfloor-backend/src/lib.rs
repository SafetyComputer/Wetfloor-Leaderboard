pub mod models;

use std::{borrow::Borrow, ops::Deref};

use actix_web::{web, get, post, HttpResponse, Responder};
use diesel::{query_dsl::UpdateAndFetchResults, r2d2::{ConnectionManager, Pool}, MysqlConnection};

#[derive(Clone)]
pub struct Dbpool {
    pub pool: Pool<ConnectionManager<MysqlConnection>>
}

impl Dbpool {
    pub fn from(database_url:&str) -> Dbpool {
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = Pool::builder().build(manager).expect("unable to connect to database");
        Dbpool {
            pool
        }
    }
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/")]
async fn get_user(db: web::Data<Dbpool>, query: web::Query<models::Player>) -> impl Responder {
    let connection= &mut db.pool.get().expect("unable to get connection");

    let result = models::Player {
        id: 1,
        name: String::from("abc"),
        elo: 123
    };
    web::Json(result)
}