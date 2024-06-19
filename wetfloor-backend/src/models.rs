use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use crate::schema;

#[derive(Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = schema::players)]
pub struct Player {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub name: String,
    #[diesel(deserialize_as = i32)]
    pub elo: Option<i32>
}