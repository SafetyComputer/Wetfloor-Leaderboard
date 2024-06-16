use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use crate::schema;

#[derive(Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = schema::players)]
pub struct Player{
    pub id: i32,
    pub name: String,
    pub elo: i32
}