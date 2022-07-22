use super::models::account::Account;
use rocket_db_pools::Connection;
use super::Users;

#[get("/accounts")]
pub async fn all_accounts(conn: Connection<Users>) -> String {
    let account = Account::get_all(conn).await;
    return account.iter().map(|account| account.to_string() + "\n").collect::<Vec<String>>().concat();
}


#[get("/account/<id>")]
pub async fn show_account(conn: Connection<Users>, id:u32) -> String {
    let account = Account::get(conn, id).await;
    return account.iter().map(|account| account.to_string() + "\n").collect::<Vec<String>>().concat();
}

#[cfg(test)]
mod tests {
    use super::super::rocket;    
    use rocket::local::blocking::Client;
    use rocket::http::Status;


    #[test]
    fn gets_response_from_accounts() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::all_accounts)).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    
    #[test]
    fn gets_response_from_account_id() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::show_account(id=1))).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
