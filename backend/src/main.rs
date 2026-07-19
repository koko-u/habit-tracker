use actix_web::App;
use actix_web::HttpServer;
use backend::config::ServerConfig;
use clap::Parser;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = ServerConfig::parse();

    HttpServer::new(|| App::new().configure(backend::configure))
        .bind((config.host, config.port))?
        .run()
        .await
}
