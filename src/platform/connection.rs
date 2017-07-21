use postgres::{Connection, TlsMode};
#[cfg(feature = "with-openssl")]
use postgres::tls::openssl::OpenSsl;
use openssl::ssl::{SslMethod, SslConnectorBuilder};
use openssl::x509::X509_FILETYPE_PEM;

use platform::config::DatabaseConfig;

pub fn connect(db: DatabaseConfig) -> Connection {
    // Configure the connection string
    let connection_string = format!(
        "postgres://{a}@{b}:{c}/{d}",
        a = db.user,
        b = db.host,
        c = db.port,
        d = db.name);
    
    // Configure SSL
    let mut ssl_builder = SslConnectorBuilder::new(SslMethod::tls()).unwrap();
    ssl_builder
        .builder_mut()
        .set_ca_file(db.ca_file)
        .unwrap();
    ssl_builder
        .builder_mut()
        .set_certificate_file(db.cert_file, X509_FILETYPE_PEM)
        .unwrap();
    ssl_builder
        .builder_mut()
        .set_private_key_file(db.cert_key_file, X509_FILETYPE_PEM)
        .unwrap();
    
    let negotiator = OpenSsl::from(ssl_builder.build());

    // Create connection to the database
    Connection::connect(
        connection_string,
        TlsMode::Require(&negotiator)).unwrap()
}
