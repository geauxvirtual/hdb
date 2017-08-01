use uuid::Uuid;

pub mod users;
pub mod tokens;

fn uuid(b: Vec<u8>) -> Uuid {
    Uuid::from_bytes(&b).unwrap()
}
