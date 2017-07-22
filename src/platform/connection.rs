use postgres::{Connection, TlsMode};
#[cfg(feature = "with-openssl")]
use postgres::tls::openssl::OpenSsl;
use openssl::ssl::{SslMethod, SslConnectorBuilder};
use openssl::x509::X509_FILETYPE_PEM;

use platform::Config;

pub fn connect(db: Config) -> Connection {
    // Configure the connection string
    let cs = connection_string(
        db.user,
        db.host,
        db.port,
        db.name);
    
    // Configure SSL
    let negotiator = ssl_builder(
        db.cert_file,
        db.cert_key_file,
        db.ca_file);

    // Create connection to the database
    Connection::connect(
        cs,
        TlsMode::Require(&negotiator)).unwrap()
}

use r2d2_postgres::PostgresConnectionManager;
use r2d2_postgres::TlsMode as R2d2TlsMode;

pub fn connection_manager(db: Config) -> PostgresConnectionManager {
    let cs = connection_string(
        db.user,
        db.host,
        db.port,
        db.name);

    let negotiator = ssl_builder(
        db.cert_file,
        db.cert_key_file,
        db.ca_file);

    PostgresConnectionManager::new(
        cs,
        R2d2TlsMode::Require(Box::new(negotiator)))
        .unwrap()
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
