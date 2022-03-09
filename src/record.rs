use crate::prelude::*;

pub trait RecordCheck: PartialEq + Default + Display {}
impl<T> RecordCheck for T where T: PartialEq + Default + Display {}

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
