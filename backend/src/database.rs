use config::{Config, File, FileFormat};
use sqlx::postgres::{PgPool, PgPoolOptions};

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct DBconf {
    pub user: String,
    pub dbname: String,
    pub host: String,
    pub port: Option<u32>,
    pub password: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Conf {
    pub database: DBconf,
}

pub async fn get_pool(
    conf_path: &std::path::PathBuf,
) -> Result<PgPool, Box<dyn std::error::Error>> {
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

    let db_conf = conf.database;

    let conn_str = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_conf.user,
        db_conf.password.unwrap_or_default(),
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
