use crate::conf::DBConf;
use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn get_pool(db_conf: &DBConf) -> Result<PgPool, Box<dyn std::error::Error>> {
    let conn_str = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_conf.user,
        db_conf.password.clone().unwrap_or_default(),
        db_conf.host,
        db_conf.port.unwrap_or(5432).to_string(),
        db_conf.dbname
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&conn_str)
        .await?;

    Ok(pool)
}
