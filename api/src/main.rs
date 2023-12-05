use rocket::http::Method;
use rocket::{get, routes};

#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
use rocket_cors::{AllowedHeaders, AllowedOrigins};

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
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:3000", "https://devhiive.github.io"]);
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }.to_cors().unwrap();

    rocket::build()
        .mount("/", routes![index, idtest])
        .attach(Template::fairing())
        .attach(cors)
}