#![feature(proc_macro_hygiene, decl_macro)]

mod img_utls;

#[macro_use] extern crate rocket;


#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[tokio::main]
async fn main() {

    println!("{:?}",img_utls::label_image("a.jpg"));
    rocket::ignite().mount("/", routes![hello]).launch();
    // // async {

    // // };

}