use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    use wetfloor_backend;
    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = wetfloor_backend::Dbpool::from(&database_url);
    HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone()))
            .service(wetfloor_backend::get_player)
            .service(wetfloor_backend::post_player)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
