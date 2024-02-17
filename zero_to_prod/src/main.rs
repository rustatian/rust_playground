use sqlx::{Connection, PgConnection, PgPool};
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("failed to read configuration");
    // let connection = PgConnection::connect(&configuration.database.connection_string()).await.expect("failed to connect to Postgres");
    //
    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address).expect("failed to bind random port");

    let configuration = get_configuration().expect("failed to read configuration");
    let connection_string = configuration.database.connection_string();

    let connection_pool = PgPool::connect(&connection_string)
        .await
        .expect("failed to connect to Postgres");

    run(listener, connection_pool)?.await
}
