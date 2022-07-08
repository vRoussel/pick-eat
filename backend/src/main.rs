use actix_web::{middleware::Logger, web, App, HttpServer};
use clap::Parser;
use log::*;
use simplelog::*;

mod database;
mod handlers;
mod query_params;
mod resources;
mod utils;

use database::Pool;

async fn start_web_server(db_pool: Pool) -> std::io::Result<()> {
    let mydata = web::Data::new(db_pool);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(mydata.clone())
            .service(
                web::scope("/v1")
                    .configure(handlers::recipes::config)
                    .configure(handlers::ingredients::config)
                    .configure(handlers::tags::config)
                    .configure(handlers::categories::config)
                    .configure(handlers::units::config) //            .configure(resources::search::config)
                    .configure(handlers::seasons::config),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

fn setup_logging() {
    let pickeat_log_config = ConfigBuilder::new()
        .set_location_level(LevelFilter::Error)
        .add_filter_allow_str("pick_eat")
        .build();

    let others_log_config = ConfigBuilder::new()
        .add_filter_ignore_str("pick_eat")
        .build();

    let init_log = CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Trace,
            pickeat_log_config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        TermLogger::new(
            LevelFilter::Warn,
            others_log_config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
    ]);
    if let Err(_) = init_log {
        CombinedLogger::init(vec![
            SimpleLogger::new(LevelFilter::Trace, pickeat_log_config.clone()),
            SimpleLogger::new(LevelFilter::Warn, others_log_config.clone()),
        ])
        .expect("Could not setup logging");
    }
}

#[derive(Parser)]
struct Cli {
    #[clap(short = 'c', long = "conf")]
    conf: std::path::PathBuf,
}

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logging();
    info!("Starting");
    let args = Cli::parse();
    let db_pool = database::get_pool(&args.conf).await?;
    start_web_server(db_pool).await?;
    Ok(())
}
