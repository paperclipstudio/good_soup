use rocket_db_pools::sqlx;
use std::fmt;
use sqlx::Row;
use sqlx::FromRow;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User{
    id: u32,
    name: String,
    age: i32,
}


#[rocket::async_trait]
impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for User {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(
            User {
                id: row.try_get("id")?,
                name: row.try_get("name")?,
                age: row.try_get("age")?,
            }
        )
    }
}
impl std::fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>)-> fmt::Result {
        write!(f, "{} : {} : {} years old", self.id, self.name, self.age)
    }
}
