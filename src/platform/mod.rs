use postgres::{Connection, TlsMode};
#[cfg(feature = "with-openssl")]
use postgres::tls::openssl::OpenSsl;

use r2d2;
use r2d2_postgres::PostgresConnectionManager;
use r2d2_postgres::TlsMode as R2d2TlsMode;

pub mod models;
pub mod config;

use self::config::Builder;

pub type PlatformConnection = Connection;
pub type Pool = r2d2::Pool<PostgresConnectionManager>;
pub type PoolConnection = r2d2::PooledConnection<PostgresConnectionManager>;

pub struct Database {
    connection_string: String,
    ssl: OpenSsl,
    max_pool_size: u32,
}

impl Database {
    pub fn builder() -> Builder {
        Builder::new()
    }

    pub fn connect(self) -> Connection {
        Connection::connect(
            self.connection_string,
            TlsMode::Require(&self.ssl)).unwrap()
        }

    pub fn pool (self) -> Pool {
        let manager = PostgresConnectionManager::new(
            self.connection_string,
            R2d2TlsMode::Require(Box::new(self.ssl)))
            .unwrap();
        // TODO Allow configuring of pool settings
        r2d2::Pool::builder()
            .max_size(self.max_pool_size)
            .build(manager)
            .unwrap()
    }
}
