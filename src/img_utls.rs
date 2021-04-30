use std::process::Command;
use rand::{distributions::Alphanumeric, Rng};
use mongodb::Client;
use bson::doc;
use std::io::Write;
use actix_multipart::{Multipart, Field};
use actix_web::{web, Error};
use futures::{StreamExt, TryStreamExt};

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

pub async fn handle_image_upload(file_data: awmp::File, title: String) -> Result<String, Error> {
    // MongoDB stuff - do in wrapper later
    let client = Client::with_uri_str(&secret::DB_S).await.unwrap();
    let db = client.database("imgboard");
    let coll = db.collection("images");

    let random_bytes: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    let filepath = format!("./uploads/{}_{}", random_bytes,file_data.sanitized_file_name());
    file_data.into_inner().persist(filepath.as_str()).map(|_| filepath.as_str()).unwrap();

    let img_labels = label_image(filepath.as_str());

    let doc = doc! {
        "img_path": filepath.as_str(),
        "lables" : img_labels,
        "title": title,
        "comments" : doc! {}
    };

    coll.insert_one(doc, None).await.unwrap();
    println!("{:#?}", filepath);

    Ok(filepath)
}