use bb8;
use bb8_postgres::PostgresConnectionManager;
use config::{Config, File, FileFormat};
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

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Conf {
    pub database: DBconf,
}

pub async fn get_pool(conf_path: &std::path::PathBuf) -> Result<Pool, Box<dyn std::error::Error>> {
    let mut _conf = Config::builder()
        .add_source(File::new(
            &conf_path.display().to_string(),
            FileFormat::Toml,
        ))
        .build()
        .expect(format!("no such file: {}", conf_path.display()).as_str());

    let conf: Conf = _conf
        .try_deserialize()
        .expect("Error while loading db conf");

    let db_conf = &conf.database;
    let mut conn_str = format!(
        "host={} user={} dbname={} port={}",
        db_conf.host, db_conf.user, db_conf.dbname, db_conf.port
    );
    if let Some(ref pwd) = db_conf.password {
        conn_str.push_str(&format!(" password={}", pwd));
    }
    let manager = PostgresConnectionManager::new_from_stringlike(conn_str, NoTls).unwrap();

    bb8::Builder::new()
        .build(manager)
        .await
        .map_err(|e| e.into())
}
