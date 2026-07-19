use actix_web::App;
use actix_web::HttpServer;
use clap::Parser;

#[derive(Parser)]
struct ServerConfig {
    #[arg(long, env = "SERVER_HOST", default_value = "127.0.0.1")]
    host: String,

    #[arg(long, env = "SERVER_PORT", default_value_t = 8000)]
    port: u16,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = ServerConfig::parse();

    HttpServer::new(|| App::new().configure(backend::configure))
        .bind((config.host, config.port))?
        .run()
        .await
}
