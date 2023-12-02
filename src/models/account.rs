use rocket_db_pools::sqlx::{FromRow, query_as, Error, sqlite, query};
use rocket_db_pools::Connection;
use rocket_db_pools::sqlx;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use super::super::Users;
use rocket::futures::StreamExt;

enum AccountType {
    Renter,
    Letter,
    Admin
}

#[derive(sqlx::FromRow)]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Account {
    id:i32,
    email:String,
    password:String, 
    salt:Option<i32>
}

impl Account {
    pub async fn get(mut conn: Connection<Users>, id:u32) -> Option<Account> {
        Some(query!("Select * from Accounts where id = ?")
            .bind(id)
            .fetch(&mut *conn).await
            .expect("Couldn't get Accounts"))
    }
    

    pub async fn get_all(mut conn: Connection<Users>) -> Vec<Account> {
        query!("Select * from Accounts")
            .fetch(&mut *conn)
            .collect()
    }

    pub async fn verify(mut conn: Connection<Users>, email:&str, password:&str) -> bool {
        println!("{}, {}",email, password);
        let result:Result<Account, _> = query_as!(Account, "Select * from Accountss where Email=? and Password=?")
            .bind(email)
            .bind(password)
            .fetch_one(&mut *conn).await;
        return match result {
            Ok(_) => true,
            Err(e) => {
                println!("{}", e.to_string());
                return false
            }
        };
    }

    pub fn to_string(&self) -> String {
        format!("id:{}, un:{}, pw:{}, salt:{:?}", 
                self.id, 
                self.email, 
                self.password, 
                self.salt
                )
    }
}

/*
impl<'r> FromRow<'r, sqlite::SqliteRow> for Account {
    fn from_row(row: &'r sqlite::SqliteRow) -> Result<Self, Error> {
        Ok(
            Account {
                id: row.try_get("Id")?,
                email: row.try_get("Email")?,
                password: row.try_get("Password")?,
                salt: row.try_get("Salt")?
            }
        )
    }
}
*/
