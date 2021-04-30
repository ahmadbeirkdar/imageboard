use actix_multipart::{Multipart, Field};
use actix_web::{HttpResponse, web};
use futures::{TryStreamExt, StreamExt};
use std::convert::TryInto;
use std::mem::swap;
use crate::DB::DB;
use std::sync::Mutex;
// use mime::Mime;

#[path = "img_utls.rs"] mod img_utls;

pub async fn save_file(data: web::Data<Mutex<DB>>,mut parts: awmp::Parts) -> HttpResponse {
    let file_data = parts.files.take("file").pop().unwrap();
    let title = String::from(*parts.texts.as_hash_map().get("title").unwrap());

    let response = img_utls::handle_image_upload(data,file_data,title).await.unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(response)
}

pub fn upload_img() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body><center>
            <form target="/upload" method="post" enctype="multipart/form-data">
                <label for="title">Picture Title:</label>
                <input type="text" id="title" name="title"><br><br>
                <input type="file" name="file" accept="image/png, image/jpeg"/>
                <button type="submit">Submit</button>
            </form></center>
        </body>
    </html>"#;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}