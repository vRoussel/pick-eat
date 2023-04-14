use config::{Config, File, FileFormat};

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DBConf {
    pub user: String,
    pub dbname: String,
    pub host: String,
    pub port: Option<u32>,
    pub password: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RedisConf {
    pub host: String,
    pub port: Option<u32>,
    pub password: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionsConf {
    pub cookie_secret: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmailConf {
    pub api_key: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Conf {
    pub database: DBConf,
    pub redis: RedisConf,
    pub sessions: SessionsConf,
    pub email: EmailConf,
}

pub fn parse_conf(conf_path: &std::path::PathBuf) -> Conf {
    let mut _conf = Config::builder()
        .add_source(File::new(
            &conf_path.display().to_string(),
            FileFormat::Toml,
        ))
        .build()
        .expect(format!("no such file: {}", conf_path.display()).as_str());

    _conf
        .try_deserialize()
        .expect("Error while loading db conf")
}
