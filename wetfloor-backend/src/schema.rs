// @generated automatically by Diesel CLI.

diesel::table! {
    player (id) {
        id -> Integer,
        #[max_length = 20]
        name -> Varchar,
        elo -> Integer,
    }
}
