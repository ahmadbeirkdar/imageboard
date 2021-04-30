use std::process::Command;
use rocket::Data;
use rand::{distributions::Alphanumeric, Rng};
use mongodb::Client;

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

pub fn handle_image_upload(img_data: Data, img_type: &str, uid: &str) -> String {
    let random_bytes: String = rand::thread_rng()
                                .sample_iter(&Alphanumeric)
                                .take(16)
                                .map(char::from)
                                .collect();
    let img_path = format!("uploads/{}_{}.{}",uid,random_bytes,img_type);
    img_data.stream_to_file(img_path.as_str());

    let img_labels = label_image(img_path.as_str());

    // MongoDB stuff - do in wrapper later
    let client = Client::with_uri_str(&secret::DB_S).await?;
    let db = client.database("imgboard");
    let coll = db.collection("images");

    let doc = doc! {
        "img_path": image_path,
        "lables" : img_labels,
        "uid": uid,
    };

    let result = coll.insert_one(doc, None).await?;
    println!("{:#?}", result);

    return img_path;

}