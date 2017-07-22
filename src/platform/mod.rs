pub mod connection;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_db_name")]
    pub(crate) name: String,

    #[serde(default = "default_db_user")]
    pub user: String,

    #[serde(default = "default_db_host")]
    pub host: String,

    #[serde(default = "default_db_port")]
    pub port: u64,

    #[serde(default = "default_db_cert_file")]
    pub cert_file: String,

    #[serde(default = "default_db_cert_key_file")]
    pub cert_key_file: String,

    #[serde(default = "default_db_ca_file")]
    pub ca_file: String,
}

impl Config {
    pub fn default() -> Config {
        Config {
            name: default_db_name(),
            user: default_db_user(),
            host: default_db_host(),
            port: default_db_port(),
            cert_file: default_db_cert_file(),
            cert_key_file: default_db_cert_key_file(),
            ca_file: default_db_ca_file(),
        }
    }
}

fn default_db_name() -> String {
    "hplat".to_string()
}

fn default_db_user() -> String {
    "root".to_string()
}

fn default_db_host() -> String {
    "127.0.0.1".to_string()
}

fn default_db_port() -> u64 {
    26257
}

fn default_db_cert_file() -> String {
    "".to_string()
}

fn default_db_cert_key_file() -> String {
    "".to_string()
}

fn default_db_ca_file() -> String {
    "".to_string()
}
