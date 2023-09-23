use actix_identity::IdentityMiddleware;
use actix_session::config::CookieContentSecurity;
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

use crate::api;
use crate::conf::{RedisConf, SessionsConf};

pub async fn start_web_server(
    pickeat_app: crate::app::App,
    sessions_conf: SessionsConf,
    redis_conf: RedisConf,
) -> std::io::Result<()> {
    let app_data = web::Data::new(pickeat_app);
    let secret_key = Key::from(sessions_conf.cookie_secret.as_bytes());
    let cookie_secure = sessions_conf.cookie_secure;

    let redis_conn_str = format!(
        "redis://:{}@{}:{}",
        redis_conf.password.unwrap_or_default(),
        redis_conf.host,
        redis_conf.port.unwrap_or(6379).to_string(),
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
                    .cookie_secure(cookie_secure)
                    .build(),
            )
            .app_data(app_data.clone())
            .service(
                web::scope("/v1")
                    .configure(api::handlers::tags::config)
                    .configure(api::handlers::categories::config)
                    .configure(api::handlers::seasons::config)
                    .configure(api::handlers::diets::config)
                    .configure(api::handlers::units::config)
                    .configure(api::handlers::ingredients::config)
                    .configure(api::handlers::accounts::config)
                    .configure(api::handlers::sessions::config)
                    .configure(api::handlers::tokens::config)
                    .configure(api::handlers::recipes::config),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
