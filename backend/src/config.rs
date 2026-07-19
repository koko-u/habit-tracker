use clap::Parser;

#[derive(Parser)]
pub struct ServerConfig {
    #[arg(long, env = "SERVER_HOST", default_value = "127.0.0.1")]
    pub host: String,

    #[arg(long, env = "SERVER_PORT", default_value_t = 8000)]
    pub port: u16,
}
