use std::process::Command;
use rand::{distributions::Alphanumeric, Rng};
use mongodb::{
    bson::{doc},
};
use actix_web::{web, Error};
use crate::DB::DB;
use std::sync::Mutex;
use std::thread;

#[path = "secret.rs"] mod secret;

pub fn label_image(img_path: &str) -> Vec<String> {
    let output = Command::new("python3")
        .arg("src/google_api.py").arg(img_path).arg("clientsecret.json").output().expect("failed to execute process");

    if output.status.success() {
        return String::from_utf8_lossy(&output.stdout)
            .replace('\n',"").split('|')
            .into_iter().map(|x| String::from(x))
            .collect::<Vec<String>>();
    }
    return vec!();
}

pub async fn handle_image_upload(data: web::Data<Mutex<DB>>, file_data: awmp::File, title: String) -> Result<String, Error> {
    let random_bytes: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    let filepath = format!("./uploads/{}_{}", random_bytes,file_data.sanitized_file_name());
    file_data.into_inner().persist(filepath.as_str()).map(|_| filepath.as_str()).unwrap();

    let doc = doc! {
        "img_path": filepath.as_str(),
        "labels" : [],
        "title": title,
        "comments" : []
    };

    let oid = data.lock().unwrap().new_image(&doc);
    println!("{:#?}", filepath);

    // Label image
    let mv_filepath = filepath.clone();
    let mv_oid = oid.clone();
    #[allow(unused_variables)]
    let child = thread::spawn(move ||  {
        let img_labels: Vec<String> = label_image(&mv_filepath);
        data.lock().unwrap().update_label(mv_oid,img_labels).unwrap();
    });


    Ok(oid)
}