

use crate::schema::users;
#[derive(Queryable, Serialize)]
pub struct User {
    // User id
    pub id: i32,
    // Username
    pub username: String,
    // Email adress
    pub email_adress: String,
    // Hashed password
    pub password_hash: String,
    // User rating
    pub rating: i32,
    // User preferences (data which should not be queried)
    pub preferences: serde_json::value::Value
}

// Struct for inserting user into table
#[derive(Insertable)]
#[table_name="users"]
pub struct InsertableUser {
    pub username: String,
    pub email_adress: String,
    pub password_hash: String,
    pub rating: i32,
    pub preferences: serde_json::value::Value,
}