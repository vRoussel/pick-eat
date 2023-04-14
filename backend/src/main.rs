use actix_identity::IdentityMiddleware;
use actix_session::config::CookieContentSecurity;
use actix_session::{storage::RedisSessionStore, Session, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::{middleware::Logger, web, App, HttpServer};
use clap::Parser;
use conf::{parse_conf, Conf};
use log::*;
use simplelog::*;
use sqlx::postgres::PgPool;

mod conf;
mod database;
mod email;
mod handlers;
mod query_params;
mod resources;

async fn start_web_server(
    db_pool: PgPool,
    email_sender: email::EmailSender,
    conf: Conf,
) -> std::io::Result<()> {
    let db_pool_data = web::Data::new(db_pool);
    let email_sender_data = web::Data::new(email_sender);
    let secret_key = Key::from(conf.sessions.cookie_secret.as_bytes());

    let redis_conn_str = format!(
        "redis://:{}@{}:{}",
        conf.redis.password.unwrap_or_default(),
        conf.redis.host,
        conf.redis.port.unwrap_or(6379).to_string(),
    );
    let redis_store = RedisSessionStore::new(redis_conn_str).await.unwrap();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(IdentityMiddleware::default())
            .wrap(
                // create cookie based session middleware
                SessionMiddleware::builder(redis_store.clone(), secret_key.clone())
                    .cookie_content_security(CookieContentSecurity::Signed)
                    .cookie_same_site(actix_web::cookie::SameSite::Strict)
                    .cookie_secure(true)
                    .build(),
            )
            .app_data(db_pool_data.clone())
            .app_data(email_sender_data.clone())
            .service(
                web::scope("/v1")
                    .configure(handlers::recipes::config)
                    .configure(handlers::ingredients::config)
                    .configure(handlers::tags::config)
                    .configure(handlers::categories::config)
                    .configure(handlers::units::config) //            .configure(resources::search::config)
                    .configure(handlers::seasons::config)
                    .configure(handlers::diets::config)
                    .configure(handlers::accounts::config)
                    .configure(handlers::sessions::config),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn setup_logging(verbose: u8) {
    let log_level = match verbose {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    let pickeat_log_config = ConfigBuilder::new().add_filter_allow_str("pickeat").build();

    let others_log_config = ConfigBuilder::new()
        .add_filter_ignore_str("pickeat")
        .build();

    let init_log = CombinedLogger::init(vec![
        TermLogger::new(
            log_level,
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
            SimpleLogger::new(log_level, pickeat_log_config.clone()),
            SimpleLogger::new(LevelFilter::Warn, others_log_config.clone()),
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
    info!("Starting");
    let conf = parse_conf(&args.conf);
    let db_pool = database::get_pool(&conf.database).await?;
    let email_sender = email::EmailSender::new(conf.email.api_key.clone());
    start_web_server(db_pool, email_sender, conf).await?;
    Ok(())
}
