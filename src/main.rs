use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use rocket_dyn_templates::context;
use rocket::Response;
use rocket::form::Form;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> Template {
    Template::render("login", context! {id:1})
}

#[derive(FromForm)]
struct Login<'r> {
    email:&'r str,
    password:&'r str
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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, template_test, check_login, homepage])
        .attach(Template::fairing())
}


