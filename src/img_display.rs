use actix_web::{get, web, App, HttpServer, Responder, HttpResponse,Result,Error};
use actix_files::NamedFile;
use crate::DB::DB;
use std::sync::Mutex;
use bson::Document;
#[path = "static_html.rs"] mod static_html;


#[get("/img/{id}")]
pub async fn image_view(data: web::Data<Mutex<DB>>,web::Path(id): web::Path<String>) -> HttpResponse {

    match data.get_ref().lock().unwrap().get_image(&id) {
        None => {HttpResponse::Ok().content_type("text/html; charset=utf-8").body("404")}
        Some(doc) => {
            let title = doc.get("title").unwrap().as_str().unwrap();
            let img_lables = doc.get("labels").unwrap().as_array().unwrap().into_iter().map(|x| x.as_str().unwrap()).collect::<Vec<&str>>().join(", ");
            let img = format!("/get_img/{}",id);
            let head = static_html::IMAGE_VIEW_HEAD.replace("{Title}",title).replace("{img}",&img).replace("{img_lables}",&img_lables);
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
                .body(format!("{}{}{}",head,comments.into_iter().collect::<String>(),static_html::IMAGE_VIEW_FOOT))
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
