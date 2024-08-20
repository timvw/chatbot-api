use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[allow(unused)]
pub struct Thread {
    pub id: String,
}

impl From<crate::threads::model::Thread> for Thread {
    fn from(value: crate::threads::model::Thread) -> Self {
        let id = value.id.to_string();
        Thread { id }
    }
}

impl From<&crate::threads::model::Thread> for Thread {
    fn from(value: &crate::threads::model::Thread) -> Self {
        let id = value.id.to_string();
        Thread { id }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[allow(unused)]
pub struct Message {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[allow(unused)]
pub struct AddMessageRequest {
    pub content: String,
}
