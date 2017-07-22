extern crate postgres;
#[cfg(feature = "with-openssl")]
extern crate openssl;

extern crate serde;
#[macro_use] extern crate serde_derive;

extern crate r2d2_postgres;

pub mod platform;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
