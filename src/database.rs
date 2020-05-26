use std::time::Duration;

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
