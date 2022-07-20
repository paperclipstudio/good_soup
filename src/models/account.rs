use rocket_db_pools::sqlx::{Row, mysql, query_as, FromRow, Error };
use rocket_db_pools::Connection;
use super::super::Users;

#[allow(dead_code)]
enum AccountType {
    Renter,
    Letter,
    Admin
}

#[allow(dead_code)]
pub struct Account {
    id:i32,
    email:String,
    password:String,
    salt:Option<i32>,
}

impl Account {
    pub async fn get(mut conn: Connection<Users>, id:u32) -> Option<Account> {
        Some(query_as("Select * from Account where id = ?")
            .bind(id)
            .fetch_one(&mut *conn).await
            .expect("Couldn't get Account"))
    }
    

    pub async fn get_all(mut conn: Connection<Users>) -> Vec<Account> {
        Some(query_as("Select Email, Password from Account")
            .fetch_all(&mut *conn).await
            .expect("Couldn't get Account")).unwrap()
    }

    pub async fn verify(mut conn: Connection<Users>, email:&str, password:&str) -> bool {
        println!("{}, {}",email, password);
        let result:Result<Account,_> = query_as("Select * from Accounts where Email=? and Password=?")
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
        format!("id:{}, un:{}, pw:{}", self.id, self.email, self.password)
    }
}


impl FromRow<'_, mysql::MySqlRow> for Account {
    fn from_row(row: &mysql::MySqlRow) -> Result<Self, Error> {
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

