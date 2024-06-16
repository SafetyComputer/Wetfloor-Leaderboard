use diesel::{deserialize::Queryable, prelude::Insertable};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Queryable, Insertable)]
pub struct Player{
    pub id: i32,
    pub name: String,
    pub elo: i32
}