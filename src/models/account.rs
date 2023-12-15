use rocket_db_pools::sqlx::{FromRow, query_as, Error, sqlite, query, Row};
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
        let result: Account = query("Select * from Accounts where id = ?")
            .bind(id)
            .fetch_one(&mut **conn)
            .await
            .as_ref()
            .and_then(|r| Ok(Account::from_row(r)))
            .unwrap().unwrap();
        return Some(result);
            
    }
    

    pub async fn get_all(mut conn: Connection<Users>) -> Vec<Account> {
        query_as::<_, Account>("Select * from Accounts")
            .fetch_all(&mut **conn)
            .await
            .unwrap_or(Vec::new())
    }

    pub async fn verify(mut conn: Connection<Users>, email:&str, password:&str) -> bool {
        println!("{}, {}",email, password);
        let result:Result<Account, _> = query_as("Select * from Accounts where Email=? and Password=?")
            .bind(email)
            .bind(password)
            .fetch_one(&mut **conn).await;
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
