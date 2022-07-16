use rocket_db_pools::sqlx;
use std::fmt;
use sqlx::Row;

#[derive(Debug)]
pub struct User{
    id: u32,
    name: String,
    age: i32,
}

impl sqlx::FromRow<'_, sqlx::mysql::MySqlRow> for User {
    fn from_row(row: &sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
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
