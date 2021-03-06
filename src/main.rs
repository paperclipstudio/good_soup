#[macro_use] extern crate rocket;
//pub mod schema;
use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use rocket_dyn_templates::context;
use rocket::form::Form;
use rand::Rng;
use rocket_db_pools::{Database, sqlx, Connection};    
mod models;
use models::user::User;
mod account;
use crate::account::*;

#[get("/")]
fn index() -> Template {
    Template::render("login", context! {id:1})
}

#[derive(Database)]
#[database("users")]
pub struct Users(sqlx::MySqlPool);

#[derive(Database)]
#[database("test")]
pub struct Test(sqlx::MySqlPool);

#[get("/user/<id>")]
async fn show_user(mut conn: Connection<Users>, id:u32) -> String {
    match sqlx::query_as::<_, User>("SELECT * FROM USERS where id=?")
        .bind(id)
        .fetch_one(&mut *conn).await
        .ok() {
            Some(user) => format!("{user}"),
            None => String::from("no user found")
        }
}

#[get("/users")]
async fn all_users(mut conn: Connection<Users>) -> String {
    let users:Vec<User> = sqlx::query_as("SELECT * FROM USERS").fetch_all(&mut *conn).await.ok().unwrap();
    return users.iter().map(|user| user.to_string() + "\n").collect::<Vec<String>>().concat()
}

#[derive(FromForm)]
struct Login<'r> {
    email:&'r str,
    password:&'r str
}

#[get("/random/<max>")]
fn random_int(max:u32) -> String {
    let mut rng = rand::thread_rng();
    let value:i32 = rng.gen_range(0,max as i32);
    return format!("This is a value between 0 and {}\n-> {}", max, value.to_string());
}

#[post("/login", data="<login>")]
async fn check_login(conn: Connection<Users>, login:Form<Login<'_>>) -> Redirect {
    return if models::account::Account::verify(conn, login.email, login.password).await {
        println!("You have logged in");
        Redirect::to("/homepage")
    } else {
        println!("Login failed for {}", login.email);
        Redirect::to("/")
    }
}


#[get("/homepage")]
fn homepage() -> Template {
    Template::render("homepage", context! {
        id: 1,
    })
}


#[get("/test")]
fn template_test() -> Template {
    Template::render("test", context! {
        title: "Hello World"
    })
}


#[get("/ping")]
fn ping() -> String {
    return String::from("pong")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
                          all_users, show_user, 
                          all_accounts, 
                          show_account,
                          random_int, index, template_test, check_login, homepage, ping])
        .attach(Template::fairing())
        .attach(Users::init())
}


