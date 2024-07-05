use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;
use std::env;
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    use wetfloor_backend;
    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = wetfloor_backend::Dbpool::from(&database_url);
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://wetfloor.dafenci.cc")
            .allow_any_header()
            .allow_any_method();
        let service = App::new().app_data(web::Data::new(pool.clone()))
            .service(wetfloor_backend::get_player)
            .service(wetfloor_backend::post_player)
            .service(wetfloor_backend::get_match)
            .service(wetfloor_backend::post_match)
            .service(wetfloor_backend::delete_match)
            .wrap(cors);
        service
    })
    .bind(("0.0.0.0", 35668))?
    .run()
    .await
}
