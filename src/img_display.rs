use actix_web::{get, web, HttpResponse};
use actix_files::NamedFile;
use crate::DB::DB;
use std::sync::Mutex;
use serde::Deserialize;
use chrono;
#[path = "static_html.rs"] mod static_html;


pub async fn image_view(data: web::Data<Mutex<DB>>,web::Path(id): web::Path<String>) -> HttpResponse {

    match data.lock().unwrap().get_image(&id) {
        None => {HttpResponse::Ok().content_type("text/html; charset=utf-8").body("404")}
        Some(doc) => {
            // Data from DB doc
            let title = doc.get("title").unwrap().as_str().unwrap();
            let mut img_lables = doc.get("labels").unwrap()
                                        .as_array().unwrap().into_iter()
                                        .map(|x| x.as_str().unwrap())
                                        .collect::<Vec<&str>>()
                                        .join(", ");
            let img = format!("/get_img/{}",id);
            if img_lables.is_empty() {
                img_lables = "In progress...".to_string();
            }

            // Parts of static html.
            let head = static_html::IMAGE_VIEW_HEAD
                                    .replace("{Title}",title)
                                    .replace("{img}",&img)
                                    .replace("{img_lables}",&img_lables);
            let footer = static_html::IMAGE_VIEW_FOOT
                                    .replace("{oid}",&id);

            // Populate docs
            let comments_docs = doc.get("comments").unwrap().as_array().unwrap();
            let mut comments: Vec<String> = vec!();
            for i in comments_docs {
                let doc = i.as_document().unwrap();
                comments.push(
                    static_html::get_comment(
                        doc.get("author").unwrap().as_str().unwrap(),
                        doc.get("date").unwrap().as_str().unwrap(),
                        doc.get("text").unwrap().as_str().unwrap()
                    )
                )
            }

            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(format!("{}{}{}",head,comments.into_iter().collect::<String>(),footer))
        }
    }

}

#[get("/get_img/{id}")]
pub async fn show_img(data: web::Data<Mutex<DB>>, web::Path(id): web::Path<String>) -> NamedFile  {
    return match data.get_ref().lock().unwrap().get_image(&id) {
        None => {
            NamedFile::open("./static/placeholder.jpg").unwrap()
        }
        Some(doc) => {
            NamedFile::open(doc.get("img_path").unwrap().as_str().unwrap()).unwrap()
        }
    }
}


#[derive(Deserialize)]
pub struct FormData {
    name: String,
    comment: String,
}

pub async fn new_comment(data: web::Data<Mutex<DB>>,web::Path(id): web::Path<String>,form: web::Form<FormData>) -> HttpResponse {
    data.lock().unwrap().insert_comment(&id,&form.name,&form.comment,&chrono::offset::Local::now().date().to_string()).unwrap();
    HttpResponse::Found()
        .header("Location", format!("/img/{}",id))
        .finish()
}
