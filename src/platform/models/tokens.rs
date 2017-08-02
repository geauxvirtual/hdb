use uuid::Uuid;
use postgres::Connection;

use super::*;

pub struct NewUserToken {
    pub user_id: Uuid,
    pub token: Vec<u8>,
}

#[derive(Debug)]
pub struct UserToken {
    pub id: i64,
    pub user_id: Uuid,
    pub token: Vec<u8>,
}

pub fn create(token: NewUserToken, conn: &Connection) -> bool {
    let stmt = conn
        .prepare("INSERT INTO user_tokens
        (user_id, token)
        VALUES
        ($1, $2);")
        .unwrap();
    let success = stmt.execute(&[
                 &token.user_id.as_bytes().to_vec(),
                 &token.token])
        .unwrap();
    if success == 0 {
        return false;
    }
    true
}

pub fn get_by_user_id(user_id: &Uuid, conn: &Connection) -> Result<UserToken, &'static str> {
    let stmt =  conn
        .prepare("SELECT * FROM user_tokens WHERE user_id = $1;")
        .unwrap();
    for row in &stmt.query(&[&user_id.as_bytes().to_vec()]).unwrap() {
        let uid = uuid(row.get(1));
        return Ok(
            UserToken {
                id: row.get(0),
                user_id: uid,
                token: row.get(2),
            })
    }
    Err("access_token not found")
}

// Get all tokens for a user

// Update access_token for a user
pub fn update(id: &i64, token: &Vec<u8>, conn: &Connection) -> bool {
    let stmt = conn
        .prepare("UPDATE user_tokens SET token = $1 WHERE id = $2;")
        .unwrap();
    let success = stmt.execute(&[
                               &token,
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
        .prepare("DELETE FROM user_tokens WHERE id = $1")
        .unwrap();
    let success = stmt.execute(&[&id]).unwrap();
    if success == 0 {
        false
    } else {
        true
    }
}
