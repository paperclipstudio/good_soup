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
    salt:Option<i32>
}

impl Account {

    pub async fn get(mut conn: Connection<Users>, id:u32) -> Option<Account> {
        Some(query_as("Select * from Accounts where id = ?")
            .bind(id)
            .fetch_one(&mut *conn).await
            .expect("Couldn't get Accounts"))
    }
    

    pub async fn get_all(mut conn: Connection<Users>) -> Vec<Account> {
        Some(query_as("Select * from Accounts")
            .fetch_all(&mut *conn).await
            .expect("Getting Email and passwords from Accounts")).unwrap()
    }

    pub async fn verify(mut conn: Connection<Users>, email:&str, password:&str) -> bool {
        println!("{}, {}",email, password);
        let result:Result<Account, _> = query_as("Select * from Accountss where Email=? and Password=?")
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

