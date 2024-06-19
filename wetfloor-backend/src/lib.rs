pub mod models;
pub mod schema;

use actix_web::{get, post, web, Responder};
use diesel::{insert_into, r2d2::{ConnectionManager, Pool}, MysqlConnection};
use diesel::prelude::*;
use models::Player;
use serde::Deserialize;

#[derive(Clone)]
pub struct Dbpool {
    pub pool: Pool<ConnectionManager<MysqlConnection>>
}

#[derive(Deserialize)]
struct PlayerQuery {
    id: Option<i32>,
    name: Option<String>
}

#[derive(Deserialize)]
struct PlayerPost {
    name: String,
    elo: Option<i32>
}

impl PlayerPost {
    fn to_player(self) -> Player {
        Player {
            id: None,
            name: self.name,
            elo: self.elo
        }
    }
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

#[get("/player")]
async fn get_player(db: web::Data<Dbpool>, query: web::Query<PlayerQuery>) -> impl Responder {
    use schema::players::dsl::*;
    let connection= &mut db.pool.get().expect("unable to get connection");
    let result: Vec<Player> = match &query.name {
        Some(v) => players.filter(name.eq(v)).load(connection).expect("db error"),
        None => players.filter(id.eq(query.id.expect("no query"))).load(connection).expect("db error")
    };
    
    web::Json(result)
}

#[post("/player")]
async fn post_player(db: web::Data<Dbpool>, new_player: web::Json<PlayerPost>) -> impl Responder {
    use schema::players::dsl::*;
    let connection = &mut db.pool.get().expect("unable to get connection");
    let new_player = new_player.into_inner().to_player();
    let result = insert_into(players).values(&new_player).execute(connection);
    match result {
        Ok(_) => web::Json("success"),
        Err(_) => web::Json("failed to insert")
    }
}