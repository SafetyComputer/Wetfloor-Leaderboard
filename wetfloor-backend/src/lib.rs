pub mod models;
pub mod schema;

use actix_web::{get, post, web, Responder};
use diesel::{insert_into, r2d2::{ConnectionManager, Pool, PooledConnection}, update, MysqlConnection};
use diesel::prelude::*;
use models::{Match, Player};
use serde::{Deserialize, Serialize};

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
    pub fn get_connection(&self) -> PooledConnection<ConnectionManager<MysqlConnection>> {
        self.pool.get().expect("unable to connect to database")
    }
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

#[derive(Serialize)]
struct MatchGet {
    id: i32,
    winner: Player,
    loser: Player,
    time: chrono::NaiveDateTime,
    win_points: i32,
    lose_points: i32
}

impl MatchGet {
    fn from(origin: &Match, connection: &mut PooledConnection<ConnectionManager<MysqlConnection>> ) -> MatchGet {
        use schema::players::dsl::*;
        let winner: Player = players.find(origin.winner).first(connection).expect("db error");
        let loser: Player = players.find(origin.loser).first(connection).expect("db error");
        MatchGet {
            id: origin.id.unwrap(),
            winner: winner,
            loser: loser,
            time: origin.time.unwrap(),
            win_points: origin.win_points,
            lose_points: origin.lose_points
        }
    }
}

#[derive(Deserialize)]
struct MatchQuery {
    id: Option<i32>,
    player_id: Option<i32>
}

#[get("/player")]
async fn get_player(db: web::Data<Dbpool>, query: web::Query<PlayerQuery>) -> impl Responder {
    use schema::players::dsl::*;
    let connection= &mut db.get_connection();
    let result: Vec<Player> = match &query.name {
        Some(v) => players.filter(name.eq(v)).load(connection).expect("db error"),
        None => match query.id {
            Some(v) => players.filter(id.eq(v)).load(connection).expect("db error"),
            None => players.load(connection).expect("db error")
        }
    };
    
    web::Json(result)
}

#[post("/player")]
async fn post_player(db: web::Data<Dbpool>, new_player: web::Json<PlayerPost>) -> impl Responder {
    use schema::players::dsl::*;
    let connection = &mut db.get_connection();
    let new_player = new_player.into_inner().to_player();
    let result = insert_into(players).values(&new_player).execute(connection);
    match result {
        Ok(_) => web::Json("success"),
        Err(_) => web::Json("failed to insert")
    }
}

#[get("/match")]
async fn get_match(db: web::Data<Dbpool>, query: web::Query<MatchQuery>) -> impl Responder {
    use schema::matches::dsl::*;
    let connection = &mut db.get_connection();
    let result_db: Vec<Match> = match query.id {
        Some(v) => matches.filter(id.eq(v)).load(connection).expect("db error"),
        None => match query.player_id {
            Some(v) => matches.filter(winner.eq(v).or(loser.eq(v))).load(connection).expect("db error"),
            None => matches.load(connection).expect("db error")
        }
    };
    let result: Vec<MatchGet> = result_db.iter().map(|v| -> MatchGet {MatchGet::from(v, connection)}).collect();
    web::Json(result)
}

#[post("/match")]
async fn post_match(db: web::Data<Dbpool>, new_match: web::Json<Match>) -> impl Responder {
    use schema::matches::dsl::*;
    let connection = &mut db.get_connection();
    let result = connection.transaction(|conn|{
        let new_match = new_match.into_inner();
        insert_into(matches).values(&new_match).execute(conn)?;
        let target_winner = schema::players::dsl::players.find(new_match.winner);
        let target_loser = schema::players::dsl::players.find(new_match.loser);
        let new_winner: Player = target_winner.first(conn)?;
        let new_loser: Player = target_loser.first(conn)?;
        let (elo_winner, elo_loser) = elo_update(new_winner, new_loser);
        update(target_winner).set(schema::players::dsl::elo.eq(elo_winner)).execute(conn)?;
        update(target_loser).set(schema::players::dsl::elo.eq(elo_loser)).execute(conn)
    });
    
    match result {
        Ok(_) => web::Json("success"),
        Err(_) => web::Json("failed")
    }
}

fn elo_update(winner: Player, loser: Player) -> (i32, i32) {
    let elo_winner = winner.elo.unwrap();
    let elo_loser = loser.elo.unwrap();
    let expected_winner = 1_f32 / (1_f32 + 10_f32.powf((elo_loser as f32- elo_winner as f32) / 400 as f32));
    let expected_loser = 1_f32 - expected_winner;
    
    ((elo_winner as f32 + 32_f32 * (1_f32 - expected_winner)).round() as i32,
    (elo_loser as f32 + 32_f32 * (0_f32 - expected_loser)).round() as i32)
}