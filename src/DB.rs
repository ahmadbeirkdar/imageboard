use mongodb::{sync::{Client, Collection, Database}};
use bson::{Document, Bson};
use std::collections::HashMap;
use futures::StreamExt;
use bson::doc;
use std::convert::TryInto;
use std::collections::hash_map::RandomState;
use bson::oid::ObjectId;

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

}