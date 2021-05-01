use actix_web::{HttpResponse, web};
use crate::DB::DB;
use std::sync::Mutex;

#[path = "img_utls.rs"] mod img_utls;

pub async fn save_file(data: web::Data<Mutex<DB>>,mut parts: awmp::Parts) -> HttpResponse {
    let file_data = parts.files.take("file").pop().unwrap();
    let title = String::from(*parts.texts.as_hash_map().get("title").unwrap());
    match img_utls::handle_image_upload(data,file_data,title).await {
        Ok(oid) => {
            HttpResponse::Found()
                .header("Location", format!("/img/{}",oid))
                .finish()
        }
        Err(_) => {
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body("Error uploading file, try again")
        }
    }

}

pub fn upload_img() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body><center>
            <form target="/upload" method="post" enctype="multipart/form-data">
                <label for="title">Picture Title:</label>
                <input type="text" id="title" name="title"><br><br>
                <input type="file" id="file" name="file" accept="image/png, image/jpeg">
                <input type="submit">
</form>
            </center>
        </body>
    </html>"#;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}