use crate::prelude::*;

/// The primary Godot interface to the database.
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

    // Creates a database record
    #[export]
    pub fn create(&mut self, _owner: &Node, model: String, attributes: Dictionary) -> String {
        let mut record = Record::new(model);
        let uuid = record.uuid.clone();
        record.attributes = attributes.to_json().to_string();

        self.storage.create(record).unwrap();

        uuid
    }

    // Finds a database record given a uuid
    #[export]
    pub fn find(&mut self, _owner: &Node, uuid: String) -> GodotString {
        let record = self
            .storage
            .find(|f| &f.uuid, uuid)
            .expect("Could not find record");

        let data = Dictionary::new();

        data.insert("uuid", record.uuid.clone());
        data.insert("model", record.model.clone());
        data.insert("attributes", record.attributes.clone());

        data.to_json()
    }

    // Updates a record
    #[export]
    pub fn update(&mut self, _owner: &Node, uuid: String, model: String, attributes: String) {
        let new = Record {
            uuid,
            model,
            attributes,
        };

        let uuid = new.uuid.clone();
        let original = self
            .storage
            .find(|f| &f.uuid, uuid)
            .expect("Could not find record to update")
            .clone();

        self.storage
            .update(&original, new.clone())
            .expect("Cannot update record");
    }

    // Removes a record
    #[export]
    pub fn destroy(&mut self, _owner: &Node, uuid: String, model: String, attributes: String) {
        let record = Record {
            uuid,
            model,
            attributes,
        };

        self.storage.destroy(&record).expect("Cannot remove record");
    }
}
