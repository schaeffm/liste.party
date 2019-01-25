#[macro_use]
extern crate rocket;

use rocket::response::Redirect;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/wetter")]
fn wetter() -> Redirect {
    Redirect::to("https://www.wetter.de/cms/locker-20-grad-in-den-bergen-ueberall-strenger-frost-in-deutschland-4280453.html?c=bed2")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, wetter])
        .mount("/www", routes![index])
        .launch();
}
