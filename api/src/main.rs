
#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        title: "Hello, world!",
        name: "Rocket",
    })
}

#[get("/<id>")]
fn idtest(id: usize) -> String {
    format!("Hello, world! {}", id)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, idtest])
        .attach(Template::fairing())    
}