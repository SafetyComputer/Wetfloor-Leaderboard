use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use diesel::Connection;
use dotenvy::dotenv;
use std::env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    for (k,_) in env::vars() {
        println!("{k}");
    }
    use diesel::mysql::MysqlConnection;
    let connection_url = env::var("DATABASE_URL").unwrap();
    let _connection = MysqlConnection::establish(&connection_url).unwrap();
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
