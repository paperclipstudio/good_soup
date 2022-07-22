use rocket_db_pools::{Database, sqlx, Connction};

#[derive(Database)]
#[database("test")]
pub struct Test(sqlx::MySqlPool);

/**
 * Deletes all data in Test
 */
fn emptyDatabase(conn:Connection<Test>) {
    sqlx::query!("Drop table Account");
    sqlx::query!("Drop table Users");
    sqlx::query!("Create table Users (Id int AUTO_INCREMENT, Name varchar(255), Age int, Primary Key (Id))");
    sqlx::query!("Create table Account (Id int, Email varchar(255), Password varchar(255), Salt varchar(225))"); ");

    sqlx::query!("create ");



    
}

