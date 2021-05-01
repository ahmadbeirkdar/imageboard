use mongodb::{sync::{Client, Collection, Database}};
use bson::{Document, Bson};
use std::collections::HashMap;
use futures::StreamExt;
use bson::doc;
use std::convert::TryInto;
use std::collections::hash_map::RandomState;
use bson::oid::ObjectId;
use std::borrow::Borrow;

pub struct DB {
    client: Client,
    db: Database,
    coll: Collection,
    img_data: HashMap<String,Document>
}

impl DB {
    pub fn init(secret: &str) -> Self {
        let client = Client::with_uri_str(secret).unwrap();
        let db = client.database("imgboard");
        let coll = db.collection("images");
        let mut img_data: HashMap<String,Document> = HashMap::new();

        let cursor = coll.find(doc! { }, None).unwrap();
        for result in cursor {
            match result {
                Ok(document) => {
                    img_data.insert(document.get("_id").unwrap().as_object_id().unwrap().to_hex(), document);
                }
                Err(e) => {},
            }
        }


        DB {
            client,
            db,
            coll,
            img_data
        }
    }

    pub fn new_image(&mut self, doc: &bson::Document) -> String {
        let id : String = self.coll.insert_one(doc.clone(), None).unwrap().inserted_id.as_object_id().unwrap().to_hex();
        self.img_data.insert(id.clone(),doc.clone());
        id.clone()
    }

    pub fn update_label(&mut self, id : String, labels : Vec<String>) {
        let doc = self.img_data.remove(&id).unwrap();
        let new_doc = doc! {
            "title": doc.get("title").unwrap().as_str().unwrap(),
            "img_path": doc.get("img_path").unwrap().as_str().unwrap(),
            "comments": doc.get("comments").unwrap().as_document().unwrap(),
            "labels": labels,
        };
        self.img_data.insert(id.clone(),new_doc.clone());
        self.coll.update_one(doc! {"_id": bson::oid::ObjectId::with_string(&id).unwrap()},new_doc,None);
    }

    pub fn get_image(&self, id : &str) -> Option<&Document>{
        self.img_data.get(id)
    }

}