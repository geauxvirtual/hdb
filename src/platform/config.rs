#[cfg(feature = "with-openssl")]
use postgres::tls::openssl::OpenSsl;
use openssl::ssl::{SslMethod, SslConnectorBuilder};
use openssl::x509::X509_FILETYPE_PEM;

use platform::Database;

pub struct Builder {
    pub database_name: String,
    pub user: String,
    pub host: String,
    pub port: u64,
    pub cert_file: String,
    pub cert_key_file: String,
    pub ca_file: String,
    pub max_pool_size: u32,
}

impl Default for Builder {
    fn default() -> Builder {
        Builder {
            database_name: "hplat".to_string(),
            user: "root".to_string(),
            host: "127.0.0.1".to_string(),
            port: 26257,
            cert_file: "".to_string(),
            cert_key_file: "".to_string(),
            ca_file: "".to_string(),
            max_pool_size: 10,
        }
    }
}

impl Builder {
    pub fn new() -> Builder {
        Builder::default()
    }

    pub fn database_name(mut self, database_name: &str) -> Builder {
        self.database_name = database_name.to_string();
        self
    }

    pub fn user(mut self, user: &str) -> Builder {
        self.user = user.to_string();
        self
    }

    pub fn host(mut self, host: &str) -> Builder {
        self.host = host.to_string();
        self
    }

    pub fn port(mut self, port: u64) -> Builder {
        self.port = port;
        self
    }

    pub fn cert_file(mut self, cert_file: &str) -> Builder {
        self.cert_file = cert_file.to_string();
        self
    }

    pub  fn cert_key_file(mut self, cert_key_file: &str) -> Builder {
        self.cert_key_file = cert_key_file.to_string();
        self
    }

    pub fn ca_file(mut self, ca_file: &str) -> Builder {
        self.ca_file = ca_file.to_string();
        self
    }

    pub fn max_pool_size(mut self, max_pool_size: u32) -> Builder {
        self.max_pool_size = max_pool_size;
        self
    }

    pub fn build(self) -> Database {
        let cs = connection_string(
            self.user,
            self.host,
            self.port,
            self.database_name);
        let negotiator = ssl_builder(
            self.cert_file,
            self.cert_key_file,
            self.ca_file);
        Database {
            connection_string: cs,
            ssl: negotiator,
            max_pool_size: self.max_pool_size
        }
    }


}

fn connection_string(user: String, host: String, port: u64, database_name: String) -> String {
    format!("postgres://{a}@{b}:{c}/{d}",
            a = user,
            b = host,
            c = port,
            d = database_name)
}

fn ssl_builder(cert_file: String, cert_key_file: String, ca_file: String) -> OpenSsl {
    let mut builder = SslConnectorBuilder::new(SslMethod::tls()).unwrap();
    builder.set_ca_file(ca_file).unwrap();
    builder.set_certificate_file(cert_file, X509_FILETYPE_PEM).unwrap();
    builder.set_private_key_file(cert_key_file, X509_FILETYPE_PEM).unwrap();
    OpenSsl::from(builder.build())
}
