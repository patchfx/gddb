use crate::prelude::*;
use gdnative::prelude::*;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Record {
    pub uuid: String,
    pub model: String,
    pub attributes: String,
}

impl Record {
    pub fn new(model: String) -> Self {
        let uuid = Uuid::new_v4().to_string();

        Self {
            uuid: uuid.clone(),
            model,
            attributes: "".into(),
        }
    }
}

#[derive(NativeClass)]
#[inherit(Node)]
pub struct GDDB {
    storage: Database<Record>,
}

#[methods]
impl GDDB {
    fn new(_owner: &Node) -> Self {
        let db: Database<Record> = Database::new("GAME", None, false);
        Self { storage: db }
    }

    #[export]
    pub fn create(&mut self, _owner: &Node, model: String, attributes: Dictionary) -> String {
        let uuid = Uuid::new_v4().to_string();
        let mut record = Record::new(model);
        record.attributes = attributes.to_json().to_string();

        self.storage.create(record).unwrap();

        uuid
    }

    #[export]
    pub fn find(&mut self, _owner: &Node, uuid: String) -> Dictionary<Unique> {
        let record = self.storage.find(|f| &f.uuid, uuid).unwrap();
        let json: HashMap<String, String> =
            serde_json::from_str(&record.attributes.clone()).unwrap();

        let data = Dictionary::new();

        for (key, value) in json {
            data.insert(key, value);
        }

        data
    }
}
