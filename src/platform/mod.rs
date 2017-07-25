use postgres::{Connection, TlsMode};
#[cfg(feature = "with-openssl")]
use postgres::tls::openssl::OpenSsl;
use openssl::ssl::{SslMethod, SslConnectorBuilder};
use openssl::x509::X509_FILETYPE_PEM;

use r2d2;
use r2d2_postgres::PostgresConnectionManager;
use r2d2_postgres::TlsMode as R2d2TlsMode;

pub mod models;

pub type Pool = r2d2::Pool<PostgresConnectionManager>;
pub type PoolConnection = r2d2::PooledConnection<PostgresConnectionManager>;

pub struct Database {
    conn_string: String,
    ssl: OpenSsl,
}

impl Database {
    pub fn new(c: Config) -> Database {
        let cs = connection_string(
            c.user,
            c.host,
            c.port,
            c.name);
        let negotiator = ssl_builder(
            c.cert_file,
            c.cert_key_file,
            c.ca_file);
        Database {
            conn_string: cs,
            ssl: negotiator
        }
    }

    pub fn connect(self) -> Connection {
        Connection::connect(
            self.conn_string,
            TlsMode::Require(&self.ssl)).unwrap()
        }

    pub fn pool (self) -> Pool {
        let manager = PostgresConnectionManager::new(
            self.conn_string,
            R2d2TlsMode::Require(Box::new(self.ssl)))
            .unwrap();
        // TODO Allow configuring of pool settings
        let config = r2d2::Config::default();
        r2d2::Pool::new(config, manager).unwrap()
    }
}

fn connection_string(
    user: String,
    host: String,
    port: u64,
    name: String)
    -> String {
        format!(
            "postgres://{a}@{b}:{c}/{d}",
            a = user,
            b = host,
            c = port,
            d = name)
    }

fn ssl_builder(
    cert_file: String,
    cert_key_file: String,
    ca_file: String)
    -> OpenSsl {
        let mut builder = SslConnectorBuilder::new(SslMethod::tls()).unwrap();
        builder
            .builder_mut()
            .set_ca_file(ca_file)
            .unwrap();
        builder
            .builder_mut()
            .set_certificate_file(cert_file, X509_FILETYPE_PEM)
            .unwrap();
        builder
            .builder_mut()
            .set_private_key_file(cert_key_file, X509_FILETYPE_PEM)
            .unwrap();

        OpenSsl::from(builder.build())
    }

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
