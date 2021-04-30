use std::process::Command;
use rand::{distributions::Alphanumeric, Rng};
use mongodb::Client;
use bson::doc;
use std::io::Write;
use actix_multipart::Multipart;
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

pub async fn handle_image_upload(mut payload: Multipart, uid: &str) -> Result<Vec<String>, Error> {
    // MongoDB stuff - do in wrapper later
    let client = Client::with_uri_str(&secret::DB_S).await.unwrap();
    let db = client.database("imgboard");
    let coll = db.collection("images");

    let mut paths: Vec<String> = vec!();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let random_bytes: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();

        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("./uploads/{}_{}_{}", uid, random_bytes,sanitize_filename::sanitize(&filename));

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }

        let img_path = format!("./uploads/{}_{}_{}", uid, random_bytes,sanitize_filename::sanitize(&filename));
        let img_labels = label_image(img_path.as_str());

        let doc = doc! {
            "img_path": img_path.as_str(),
            "lables" : img_labels,
            "uid": uid,
            "comments" : doc! {}
        };

        coll.insert_one(doc, None).await.unwrap();
        println!("{:#?}", img_path);
        paths.push(img_path);
    }
    Ok(paths)
}