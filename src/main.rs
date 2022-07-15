#[macro_use] extern crate rocket;
//pub mod schema;
//mod models;
use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use rocket_dyn_templates::context;
use rocket::form::Form;
use rand::Rng;
use rocket_db_pools::{Database, sqlx, Connection};    
use sqlx::Row;
use rocket::futures::TryStreamExt;


#[get("/")]
fn index() -> Template {
    Template::render("login", context! {id:1})
}

#[derive(Database)]
#[database("users")]
pub struct Users(sqlx::MySqlPool);

/*
#[derive(sqlx::)]
struct User{
    id: i32,
    name: String,
    age: i32,
}
*/


#[get("/users")]
async fn users(mut conn: Connection<Users>) -> String {
    let mut page = "start\n".to_string();
    let mut rs = sqlx::query("SELECT * FROM USERS").fetch(&mut *conn);
    let mut count = 0;
    while let Some(row) = rs.try_next().await.ok() {
        count += 1;
        if count > 10 {
            break;
        }
        let s = match row {
            Some(r) => {
                let p:u32 = r.get(0);
                let n:String = r.get(1);
                let a:i32 = r.get(2);
                format!("{}|{}|{}\n", p, n, a)
            },
            None => String::from("...")
        };
       page = page + (s.as_str());
    }
    page = page + "\nend";


    return page;
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
        .attach(Users::init())
}


