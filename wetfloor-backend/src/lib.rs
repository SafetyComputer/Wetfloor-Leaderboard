pub mod models;
pub mod schema;

use std::collections::HashMap;
use actix_web::{get, post, web, Responder, delete};
use diesel::{insert_into, r2d2::{ConnectionManager, Pool, PooledConnection}, update, MysqlConnection};
use diesel::prelude::*;
use models::{Match, Player};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Dbpool {
    pub pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl Dbpool {
    pub fn from(database_url: &str) -> Dbpool {
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
    name: Option<String>,
}

#[derive(Deserialize)]
struct PlayerPost {
    name: String,
    default_elo: Option<i32>,
}

impl PlayerPost {
    fn to_player(self) -> Player {
        Player {
            id: None,
            name: self.name,
            elo: self.default_elo,
            default_elo: self.default_elo,
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
    lose_points: i32,
}

impl MatchGet {
    fn from(origin: &Match, connection: &mut PooledConnection<ConnectionManager<MysqlConnection>>) -> MatchGet {
        use schema::players::dsl::*;
        let winner: Player = players.find(origin.winner).first(connection).expect("db error");
        let loser: Player = players.find(origin.loser).first(connection).expect("db error");
        MatchGet {
            id: origin.id.unwrap(),
            winner: winner,
            loser: loser,
            time: origin.time.unwrap(),
            win_points: origin.win_points,
            lose_points: origin.lose_points,
        }
    }
}

#[derive(Deserialize)]
struct MatchQuery {
    id: Option<i32>,
    player_id: Option<i32>,
}

#[get("/player")]
async fn get_player(db: web::Data<Dbpool>, query: web::Query<PlayerQuery>) -> impl Responder {
    use schema::players::dsl::*;
    let connection = &mut db.get_connection();
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
    let result: Vec<MatchGet> = result_db.iter().map(|v| -> MatchGet { MatchGet::from(v, connection) }).collect();
    web::Json(result)
}

#[post("/match")]
async fn post_match(db: web::Data<Dbpool>, mut new_match: web::Json<Match>) -> impl Responder {
    use schema::matches::dsl::*;
    let connection = &mut db.get_connection();

    new_match.time = match new_match.time {
        Some(v) => Some(v),
        None => Some(chrono::Local::now().naive_local())
    };

    let result: Result<(), diesel::result::Error> = connection.transaction(|conn| {
        insert_into(matches).values(&*new_match).execute(conn)?;

        // get the latest match time
        let latest_match: Match = matches.order(time.desc()).first(conn).expect("db error");
        let latest_match_time = latest_match.time.unwrap();
        let new_match_time = new_match.time.unwrap();

        // update elo
        // if the new match is older than the latest match, update all players' elo from default_elo
        // otherwise, update the winner and loser's elo from the new match
        if new_match_time < latest_match_time {
            elo_update_from_default(conn)?;
        } else {
            elo_update_from_match(conn, &new_match)?;
        };

        Ok(())
    });

    match result {
        Ok(_) => web::Json("success"),
        Err(_) => web::Json("failed to update elo")
    }
}

#[delete("/match")]
async fn delete_match(db: web::Data<Dbpool>, query: web::Query<MatchQuery>) -> impl Responder {
    use schema::matches::dsl::*;
    let connection = &mut db.get_connection();
    let result = match query.id {
        Some(v) => connection.transaction(|conn| {
            let result = diesel::delete(matches.find(v)).execute(conn)?;
            elo_update_from_default(conn)?;
            Ok(result)
        }),
        None => Err(diesel::result::Error::NotFound)
    };

    match result {
        Ok(_) => web::Json("success"),
        Err(_) => web::Json("failed to delete")
    }
}

fn elo_update(winner_elo: i32, loser_elo: i32) -> (i32, i32) {
    let expected_winner = 1_f32 / (1_f32 + 10_f32.powf((loser_elo as f32 - winner_elo as f32) / 400 as f32));
    let expected_loser = 1_f32 - expected_winner;

    ((winner_elo as f32 + 32_f32 * (1_f32 - expected_winner)).round() as i32,
     (loser_elo as f32 + 32_f32 * (0_f32 - expected_loser)).round() as i32)
}


fn elo_update_from_match(connection: &mut PooledConnection<ConnectionManager<MysqlConnection>>, new_match: &Match) -> Result<(), diesel::result::Error> {
    /// read all players from database
    /// update the winner and loser's elo from the new match

    use schema::players::dsl::*;
    let winner: Player = players.find(new_match.winner).first(connection).expect("db error");
    let loser: Player = players.find(new_match.loser).first(connection).expect("db error");
    let winner_elo = winner.elo.unwrap();
    let loser_elo = loser.elo.unwrap();
    let (new_winner_elo, new_loser_elo) = elo_update(winner_elo, loser_elo);

    update(players.find(new_match.winner)).set(schema::players::dsl::elo.eq(new_winner_elo)).execute(connection)?;
    update(players.find(new_match.loser)).set(schema::players::dsl::elo.eq(new_loser_elo)).execute(connection)?;
    Ok(())
}

fn elo_update_from_default(connection: &mut PooledConnection<ConnectionManager<MysqlConnection>>) -> Result<(), diesel::result::Error> {
    /// read all players and matches from database
    /// update all players' elo from default_elo

    use schema::players::dsl::*;
    use schema::matches::dsl::*;
    let all_players: Vec<Player> = players.load(connection).expect("db error");
    let all_matches: Vec<Match> = matches.load(connection).expect("db error");

    let mut elo_map: HashMap<i32, i32> = HashMap::new();
    for player in all_players.iter() {
        elo_map.insert(player.id.unwrap(), player.default_elo.unwrap());
    }

    for current_match in all_matches.iter() {
        let winner_elo = elo_map.get(&current_match.winner).unwrap();
        let loser_elo = elo_map.get(&current_match.loser).unwrap();
        let (new_winner_elo, new_loser_elo) = elo_update(*winner_elo, *loser_elo);
        elo_map.insert(current_match.winner, new_winner_elo);
        elo_map.insert(current_match.loser, new_loser_elo);
    }

    for (new_id, new_elo) in elo_map.iter() {
        update(players.find(new_id)).set(schema::players::dsl::elo.eq(new_elo)).execute(connection)?;
    }
    Ok(())
}