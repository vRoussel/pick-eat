extern crate actix_web;
extern crate tokio_postgres;
extern crate simplelog;
extern crate log;

use actix_web::{web, App, HttpServer};
use tokio_postgres::{NoTls, Error};
use log::*;
use simplelog::*;
use tokio;

mod resources;
mod handlers;


async fn start_web_server(db_conn: tokio_postgres::Client) -> std::io::Result<()> {
    let mydata = web::Data::new(db_conn);
    HttpServer::new(move || App::new()
        .app_data(mydata.clone())
        .service(web::scope("/v1/")
            .configure(handlers::recipes::config)
            .configure(handlers::ingredients::config)
            .configure(handlers::tags::config)
            .configure(handlers::categories::config)
            .configure(handlers::units::config)
//            .configure(resources::search::config)
        )
    )
    .bind("127.0.0.1:8080")?.run().await
}

async fn get_db_conn() -> Result<tokio_postgres::Client, tokio_postgres::Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=valentin dbname=pick_eat", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    Ok(client)
}

fn setup_logging() {
    let pickeat_log_config = ConfigBuilder::new()
        .set_location_level(LevelFilter::Error)
        .add_filter_allow_str("pick_eat")
        .build();

    let others_log_config = ConfigBuilder::new()
        .add_filter_ignore_str("pick_eat")
        .build();

    let init_log =
        CombinedLogger::init(
            vec![
                TermLogger::new(LevelFilter::Debug, pickeat_log_config.clone(), TerminalMode::Mixed).unwrap(),
                TermLogger::new(LevelFilter::Warn, others_log_config.clone(), TerminalMode::Mixed).unwrap(),
            ]
    );
    if let Err(_) = init_log {
        CombinedLogger::init(
            vec![
                SimpleLogger::new(LevelFilter::Debug, pickeat_log_config.clone()),
                SimpleLogger::new(LevelFilter::Warn, others_log_config.clone()),
            ]
        ).expect("Could not setup logging");
    }
}

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logging();
    info!("Starting");
    let db_conn = get_db_conn().await?;
    start_web_server(db_conn).await?;
    Ok(())
}
