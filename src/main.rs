#[macro_use]
extern crate rocket;

use rocket::fs::{FileServer, NamedFile};
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;
#[get("/")]
fn index() -> Template {
    Template::render("index", context! { foo: 123 })
}

#[launch]
fn rocket() -> _ {
    // The "mount" string determines the path, the route is the final section of it
    // eg build().mount("/projects")
    // Would mount to 127.0.0.1:8000/projects/<route>
    rocket::build()
        .mount("/", routes![index])
        .mount("/css/", routes![stylesheet])
        .mount("/projects/", routes![projectIndex, projectKitty])
        .mount("/public/", FileServer::from("public/"))
        .attach(Template::fairing())
}

// ------------------------------------------ Project routes ------------------------------------------
#[get("/")]
async fn projectIndex() -> Template {
    Template::render("projects/kitty", context! {})
}

#[get("/KittyGoesMeow")]
async fn projectKitty() -> Template {
    Template::render("projects/kitty", context! {})
}

#[get("/style.css")]
async fn stylesheet() -> Option<NamedFile> {
    NamedFile::open("templates/css/style.css").await.ok()
}
