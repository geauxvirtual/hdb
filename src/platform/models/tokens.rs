use chrono::{DateTime, Utc};
use postgres::Connection;

pub struct NewUserToken {
    pub username: String,
    pub token: Vec<u8>,
    pub expires: DateTime<Utc>,
}

#[derive(Debug)]
pub struct UserToken {
    pub id: i64,
    pub username: String,
    pub token: Vec<u8>,
    pub expires: DateTime<Utc>,
}

pub fn create(token: NewUserToken, conn: &Connection) -> bool {
    let stmt = conn
        .prepare("INSERT INTO user_tokens
        (username, token, expires)
        VALUES
        ($1, $2, $3);")
        .unwrap();
    let success = stmt.execute(&[
                 &token.username,
                 &token.token,
                 &token.expires])
        .unwrap();
    if success == 0 {
        return false;
    }
    true
}

pub fn get_by_token(token: &Vec<u8>, conn: &Connection) -> Result<UserToken, &'static str> {
    let stmt =  conn
        .prepare("SELECT * FROM user_tokens WHERE token = $1;")
        .unwrap();
    for row in &stmt.query(&[&token]).unwrap() {
        return Ok(
            UserToken {
                id: row.get(0),
                username: row.get(1),
                token: row.get(2),
                expires: row.get(3),
            })
    }
    Err("access_token not found")
}

pub fn get_by_username(username: &str, conn: &Connection) -> Result<UserToken, &'static str> {
    let stmt =  conn
        .prepare("SELECT * FROM user_tokens WHERE username = $1;")
        .unwrap();
    for row in &stmt.query(&[&username]).unwrap() {
        return Ok(
            UserToken {
                id: row.get(0),
                username: row.get(1),
                token: row.get(2),
                expires: row.get(3),
            })
    }
    Err("access_token not found")
}

// Get all tokens for a user

// Update access_token for a user
pub fn update(id: &i64, token: &Vec<u8>, expires: &DateTime<Utc>, conn: &Connection) -> bool {
    let stmt = conn
        .prepare("UPDATE user_tokens SET (token, expires) = ($1, $2) WHERE id = $3;")
        .unwrap();
    let success = stmt.execute(&[
                               &token,
                               &expires,
                               &id])
        .unwrap();
    if success == 0 {
        false
    } else {
        true
    }
}

// Delete access_token for a user
pub fn delete(id: &i64, conn: &Connection) -> bool {
    let stmt = conn
        .prepare("DELETE FROM tokens WHERE id = $1")
        .unwrap();
    let success = stmt.execute(&[&id]).unwrap();
    if success == 0 {
        false
    } else {
        true
    }
}
