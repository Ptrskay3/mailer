use learning_actix::config::get_config;
use learning_actix::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("failed to load config");
    let addr = format!("127.0.0.1:{}", config.port);
    let connection = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect database.");
    let listener = TcpListener::bind(addr)?;
    run(listener, connection)?.await
}
