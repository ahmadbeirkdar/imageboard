mod img_utls;
mod img_upload_handlers;
mod DB;
mod secret;

use actix_multipart::Multipart;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("./uploads").unwrap();

    let ip = "0.0.0.0:3000";

    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default()).app_data(web::Data::new(Mutex::new(DB::DB::init(secret::DB_S)))).service(
            web::resource("/upload")
                .route(web::get().to(img_upload_handlers::upload_img))
                .route(web::post().to(img_upload_handlers::save_file)),
        )
    })
        .bind(ip)?
        .run()
        .await
}