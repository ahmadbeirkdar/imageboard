use actix_multipart::Multipart;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
#[path = "img_utls.rs"] mod img_utls;

pub async fn save_file(mut payload: Multipart) -> HttpResponse {
    let response = img_utls::handle_image_upload(payload,"091237").await.unwrap().into_iter().collect::<String>();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(response)
}

pub fn upload_img() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/upload" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}