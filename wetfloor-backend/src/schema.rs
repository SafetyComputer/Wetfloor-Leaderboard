// @generated automatically by Diesel CLI.

diesel::table! {
    matches (id) {
        id -> Integer,
        winner -> Integer,
        loser -> Integer,
        time -> Datetime,
        win_points -> Integer,
        lose_points -> Integer,
    }
}

diesel::table! {
    players (id) {
        id -> Integer,
        #[max_length = 20]
        name -> Varchar,
        elo -> Integer,
        default_elo -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    matches,
    players,
);
