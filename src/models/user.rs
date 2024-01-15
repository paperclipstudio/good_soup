use rocket_db_pools::sqlx;
use std::fmt;
use sqlx::Row;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User{
    id: u32,
    name: String,
    age: u32,
}

impl User {
    pub fn as_card(&self) -> Template {
        Template::render("user/card", self)
    }
}


#[rocket::async_trait]
impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for User {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(
            User {
                id: row.try_get("id").unwrap_or(999),
                name: row.try_get("name").unwrap_or("NONE".to_string()),
                age: row.try_get("age").unwrap_or(998),
            }
        )
    }
}


impl std::fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>)-> fmt::Result {
        write!(f, "{} : {} : {} years old", self.id, self.name, self.age)
    }
}
