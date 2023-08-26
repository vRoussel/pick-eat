use clap::Parser;
use conf::parse_conf;
use log::*;
use simplelog::*;

mod api;
mod app;
mod conf;
mod database;
mod email;
mod models;
mod query_params;
mod storage;
mod utils;
mod webserver;

use app::App;
use webserver::start_web_server;

fn setup_logging(verbose: u8) {
    let pickeat_log_level = match verbose {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    let others_log_level = match verbose {
        0 => LevelFilter::Info,
        1 | 2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    let pickeat_log_config = ConfigBuilder::new().add_filter_allow_str("pickeat").build();

    let others_log_config = ConfigBuilder::new()
        .add_filter_ignore_str("pickeat")
        .build();

    let init_log = CombinedLogger::init(vec![
        TermLogger::new(
            pickeat_log_level,
            pickeat_log_config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        TermLogger::new(
            pickeat_log_level,
            others_log_config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
    ]);
    if let Err(_) = init_log {
        CombinedLogger::init(vec![
            SimpleLogger::new(pickeat_log_level, pickeat_log_config.clone()),
            SimpleLogger::new(others_log_level, others_log_config.clone()),
        ])
        .expect("Could not setup logging");
    }
}

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    conf: std::path::PathBuf,
    #[clap(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    setup_logging(args.verbose);
    info!("Starting app");
    let conf = parse_conf(&args.conf);
    let email_sender = email::EmailSender::new(conf.email.api_key.clone());
    let db_pool = database::get_pool(&conf.database).await?;
    database::add_default_data(&db_pool)
        .await
        .expect("Error while initializing DB");
    let app = App::new(db_pool, email_sender);
    start_web_server(app, conf.sessions, conf.redis).await?;
    Ok(())
}
