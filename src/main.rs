#[macro_use] extern crate rocket;

use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use rocket_dyn_templates::context;
use rocket::form::Form;
use rand::Rng;
use rocket_sync_db_pools::{database, diesel};    

#[database("users")]
struct UserDatabase(diesel::MysqlConnection);


/*fn main() {
    rocket::Ignite()
        .attach(UserDatabase.fairing())
        .launch();
}
*/

#[get("/")]
fn index() -> Template {
    Template::render("login", context! {id:1})
}

#[get("/users")]
async fn users(conn: UserDatabase) -> String {
    return conn.run(|_| String::from("wow")).await
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
fn check_login(login:Form<Login>) -> Redirect {
    return if login.email == login.password {
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
        id: 1
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
    rocket::build().mount("/", routes![users, random_int, index, template_test, check_login, homepage, ping])
        .attach(Template::fairing())
        .attach(UserDatabase::fairing())
}


