use actix_web::{get, web, HttpResponse};
use actix_files::NamedFile;
use crate::DB::DB;
use std::sync::Mutex;
use serde::Deserialize;
use chrono;
use bson::Document;
use std::collections::HashMap;

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

#[derive(Deserialize)]
pub struct SearchFormData {
    text: String,
}

pub async fn new_comment(data: web::Data<Mutex<DB>>,web::Path(id): web::Path<String>,form: web::Form<FormData>) -> HttpResponse {
    data.lock().unwrap().insert_comment(&id,&form.name,&form.comment,&chrono::offset::Local::now().date().to_string()).unwrap();
    HttpResponse::Found()
        .header("Location", format!("/img/{}",id))
        .finish()
}
fn get_index(img_data: &HashMap<String,Document>) -> String {
    let get_title = | x: Document | -> String {x.get("title").unwrap().as_str().unwrap().to_string()};

    let grid1 : String = img_data.clone().into_iter().step_by(3).map(|x| static_html::get_img_div(&get_title(x.1),&x.0) ).collect();
    let grid2 : String = img_data.clone().into_iter().skip(1).step_by(3).map(|x| static_html::get_img_div(&get_title(x.1),&x.0) ).collect();
    let grid3 : String = img_data.clone().into_iter().skip(2).step_by(3).map(|x| static_html::get_img_div(&get_title(x.1),&x.0) ).collect();

    static_html::IMAGE_INDEX_VIEW.replace("{grid1}",&grid1).replace("{grid2}",&grid2).replace("{grid3}",&grid3)
}

pub async fn index(data: web::Data<Mutex<DB>>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(get_index(&data.lock().unwrap().populate()))
}

pub async fn search(data: web::Data<Mutex<DB>>,form: web::Form<SearchFormData>) -> HttpResponse {
    let check_txt = | doc_txt : &str, txt: &str | -> bool {
        doc_txt.clone()
            .to_ascii_lowercase()
            .contains(&txt.clone().to_ascii_lowercase())
    };
    let check_labels = | doc: &Document, txt: &str | -> bool {
      doc.get("labels")
          .unwrap().as_array().unwrap()
          .into_iter()
          .any(|x| check_txt(x.as_str().unwrap(),txt))
    };
    let check_title = | doc: &Document, txt: &str | -> bool {
        check_txt(doc.get("title").unwrap().as_str().unwrap(),txt)
    };

    let img_data : HashMap<String,Document> = data.lock()
                                                    .unwrap().populate()
                                                    .into_iter()
                                                    .filter(|x| (check_title(&x.1,&form.text) || check_labels(&x.1,&form.text)))
                                                    .collect();


    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(get_index(&img_data))
}