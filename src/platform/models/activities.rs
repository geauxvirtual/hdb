use uuid::Uuid;
use postgres::Connection;

use super::*;

pub struct NewActivity {
    pub user_id: Uuid,
    pub filename: String,
    pub activity_type: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Activity{
    pub id: i64,
    pub user_id: Uuid,
    pub filename: String,
    pub activity_type: String,
    pub name: String,
}

// Create activity and return activity created.
pub fn create(activity: NewActivity, conn: &Connection) -> Result<Activity, &'static str> {
    let stmt = conn
        .prepare("INSERT INTO activities
        (user_id, filename, type, name)
        VALUES
        ($1, $2, $3, $4)
        RETURNING *;")
        .unwrap();
    for row in &stmt.query(&[
                 &activity.user_id.as_bytes().to_vec(),
                 &activity.filename,
                 &activity.activity_type.unwrap_or("".to_string()),
                 &activity.name.unwrap_or("".to_string())])
        .unwrap() {
            let uid = uuid(row.get(1));
            return Ok(
                Activity {
                    id: row.get(0),
                    user_id: uid,
                    filename: row.get(2),
                    activity_type: row.get(3),
                    name: row.get(4),
                }
            )
    }
    Err("Error creating activity")
}
