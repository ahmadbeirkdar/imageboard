mod img_utls;

use actix_multipart::Multipart;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};

async fn save_file(mut payload: Multipart) -> HttpResponse {
    let response = img_utls::handle_image_upload(payload,"091237").await.unwrap().into_iter().collect::<String>();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(response)
}

fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    std::fs::create_dir_all("./tmp").unwrap();

    let ip = "0.0.0.0:3000";

    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default()).service(
            web::resource("/")
                .route(web::get().to(index))
                .route(web::post().to(save_file)),
        )
    })
        .bind(ip)?
        .run()
        .await
}