extern crate actix_web;
extern crate tokio_postgres;
extern crate simplelog;
extern crate log;
extern crate config;

use actix_web::{web, App, HttpServer};
use bb8;
use log::*;
use simplelog::*;
use tokio_postgres::NoTls;
use tokio;

mod resources;
mod handlers;
mod utils;
mod database;

use database::Pool;


async fn start_web_server(db_pool: Pool) -> std::io::Result<()> {
    let mydata = web::Data::new(db_pool);
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
                TermLogger::new(LevelFilter::Trace, pickeat_log_config.clone(), TerminalMode::Mixed).unwrap(),
                TermLogger::new(LevelFilter::Warn, others_log_config.clone(), TerminalMode::Mixed).unwrap(),
            ]
    );
    if let Err(_) = init_log {
        CombinedLogger::init(
            vec![
                SimpleLogger::new(LevelFilter::Trace, pickeat_log_config.clone()),
                SimpleLogger::new(LevelFilter::Warn, others_log_config.clone()),
            ]
        ).expect("Could not setup logging");
    }
}

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logging();
    info!("Starting");
    let db_pool = database::get_pool().await?;
    start_web_server(db_pool).await?;
    Ok(())
}
