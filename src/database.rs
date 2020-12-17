use bb8;
use bb8_postgres::PostgresConnectionManager;
use std::time::Duration;
use tokio_postgres::tls::NoTls;

pub type Pool = bb8::Pool<PostgresConnectionManager<NoTls>>;

#[derive(Debug)]
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct DBconf {
    pub user: String,
    pub dbname: String,
    pub host: String,
    pub port: u32,
    pub password: Option<String>,
    pub idle_timeout: Option<Duration>,
    pub min_idle_conns: Option<u32>,
    pub max_pool_size: u32
}

pub async fn get_pool() -> Result<Pool, Box<dyn std::error::Error>> {
    let mut _conf = config::Config::new();
    _conf.merge(config::File::with_name("db_conf.yml"))
        .expect("db_conf.yml not found");

    let db_conf: DBconf = _conf.try_into()
        .expect("Error while loading db conf");

    let mut conn_str = format!("host={} user={} dbname={}",
        db_conf.host, db_conf.user, db_conf.dbname);
    if let Some(pwd) = db_conf.password {
        conn_str.push_str(&format!(" password={}", pwd));
    }
    let manager = PostgresConnectionManager::new_from_stringlike(conn_str, NoTls).unwrap();

    bb8::Builder::new()
        .max_size(db_conf.max_pool_size)
        .min_idle(db_conf.min_idle_conns)
        .idle_timeout(db_conf.idle_timeout)
        .build(manager)
        .await
        .map_err(|e|e.into())
}
