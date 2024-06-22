use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use crate::schema;

#[derive(Serialize, Deserialize, Insertable, Queryable, Identifiable)]
#[diesel(table_name = schema::players)]
pub struct Player {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub name: String,
    #[diesel(deserialize_as = i32)]
    pub elo: Option<i32>
}

#[derive(Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = schema::matches)]
pub struct Match {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub winner: i32,
    pub loser: i32,
    #[diesel(deserialize_as = chrono::NaiveDateTime)]
    pub time: Option<chrono::NaiveDateTime>,
    pub win_points: i32,
    pub lose_points: i32
}