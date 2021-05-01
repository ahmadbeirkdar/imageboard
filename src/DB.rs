use mongodb::{sync::{Client, Collection, Database}, error::Error};
use bson::{Document};
use std::collections::HashMap;
use bson::doc;
use bson::bson;
use mongodb::results::UpdateResult;

pub struct DB {
    client: Client,
    db: Database,
    coll: Collection,
}

impl DB {
    pub fn init(secret: &str) -> Self {
        let client = Client::with_uri_str(secret).unwrap();
        let db = client.database("imgboard");
        let coll = db.collection("images");

        DB {
            client,
            db,
            coll,
        }
    }

    pub fn new_image(&mut self, doc: &bson::Document) -> String {
        let id : String = self.coll.insert_one(doc.clone(), None).unwrap().inserted_id.as_object_id().unwrap().to_hex();
        id.clone()
    }

    pub fn update_label(&mut self, id : String, labels : Vec<String>) -> Result<UpdateResult, Error>{
        let doc = self.get_image(&id).unwrap();
        let new_doc = doc! {
            "title": doc.get("title").unwrap().as_str().unwrap(),
            "img_path": doc.get("img_path").unwrap().as_str().unwrap(),
            "comments": doc.get("comments").unwrap().as_array().unwrap(),
            "labels": labels,
        };
        self.coll.update_one(doc! {"_id": bson::oid::ObjectId::with_string(&id).unwrap_or_default()},new_doc,None)
    }

    pub fn get_image(&self, id : &str) -> Option<Document>{
        self.coll.find_one(doc! {"_id": bson::oid::ObjectId::with_string(&id).unwrap_or_default()}, None).unwrap()
    }

    pub fn insert_comment(&self, id: &str, author: &str, text: &str, date: &str) -> Result<UpdateResult, Error>{
        let old_doc = self.get_image(id).unwrap();
        let mut arr = old_doc.get("comments").unwrap().as_array().unwrap().clone();
        arr.push(
            bson! ({
                "author" : author,
                "text" : text,
                "date" : date,
            })
        );

        let new_doc = doc! {
            "title": old_doc.get("title").unwrap().as_str().unwrap(),
            "img_path": old_doc.get("img_path").unwrap().as_str().unwrap(),
            "comments": arr,
            "labels": old_doc.get("labels").unwrap().as_array().unwrap(),
        };
        self.coll.update_one(doc! {"_id": bson::oid::ObjectId::with_string(&id).unwrap_or_default()},new_doc,None)
    }

    pub fn populate(&self) -> HashMap<String,Document>{
        let mut img_data: HashMap<String,Document> = HashMap::new();

        let cursor = self.coll.find(doc! { }, None).unwrap();
        for result in cursor {
            match result {
                Ok(document) => {
                    img_data.insert(document.get("_id").unwrap().as_object_id().unwrap().to_hex(), document);
                }
                #[allow(unused_variables)]
                Err(e) => {},
            }
        }
        img_data
    }

}