//use uuid;
use chrono::{DateTime, Utc};
use postgres::Connection;

#[derive(Deserialize)]
pub struct NewUser {
    pub username: String,
    pub salt: Vec<u8>,
    pub password: Vec<u8>,
    pub active: bool,
    pub created_on: DateTime<Utc>,
}

#[derive(Debug)]
pub struct User {
    id: Vec<u8>,
    username: String,
    salt: Vec<u8>,
    password: Vec<u8>,
    active: bool,
    created_on: DateTime<Utc>,
}

impl User {
    pub fn username(self) -> String {
        self.username
    }
}

pub fn exists(username: &str, conn: &Connection) -> bool {
    let stmt = conn
        .prepare("SELECT username FROM users WHERE username = $1;")
        .unwrap();
    let is_empty = stmt.query(&[&username]).unwrap().is_empty();
    if is_empty {
        false
    } else {
        true
    }
}

pub fn create(user: NewUser, conn: &Connection) -> bool {
    let stmt = conn
        .prepare("INSERT INTO users
        (username, salt, password, active, created_on)
        VALUES
        ($1, $2, $3, $4, $5);")
        .unwrap();
    let success = stmt.execute(&[
                 &user.username,
                 &user.salt,
                 &user.password,
                 &user.active,
                 &user.created_on])
        .unwrap();
    if success == 0 {
        return false;
    }
    true
}

pub fn get_by_username(username: &str, conn: &Connection) -> Result<User, &'static str> {
    let stmt =  conn
        .prepare("SELECT * FROM users WHERE username = $1;")
        .unwrap();
    for row in &stmt.query(&[&username]).unwrap() {
        let user = User {
            id: row.get(0),
            username: row.get(1),
            salt: row.get(2),
            password: row.get(3),
            active: row.get(4),
            created_on: row.get(5),
        };
        return Ok(user);
    }
    Err("User not found")
}
