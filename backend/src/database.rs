use bb8;
use bb8_postgres::PostgresConnectionManager;
use config::{Config, File, FileFormat};
use std::time::Duration;
use tokio_postgres::tls::NoTls;

pub type Pool = bb8::Pool<PostgresConnectionManager<NoTls>>;

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct DBconf {
    pub user: String,
    pub dbname: String,
    pub host: String,
    pub port: u32,
    pub password: Option<String>,
}

pub async fn get_pool() -> Result<Pool, Box<dyn std::error::Error>> {
    let mut _conf = Config::builder()
        .add_source(File::new("db_conf.yml", FileFormat::Yaml))
        .build()
        .expect("db_conf.yml not found");

    let db_conf: DBconf = _conf
        .try_deserialize()
        .expect("Error while loading db conf");

    let mut conn_str = format!(
        "host={} user={} dbname={}",
        db_conf.host, db_conf.user, db_conf.dbname
    );
    if let Some(pwd) = db_conf.password {
        conn_str.push_str(&format!(" password={}", pwd));
    }
    let manager = PostgresConnectionManager::new_from_stringlike(conn_str, NoTls).unwrap();

    bb8::Builder::new()
        .build(manager)
        .await
        .map_err(|e| e.into())
}
