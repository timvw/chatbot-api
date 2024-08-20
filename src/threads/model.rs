use uuid::Uuid;

#[derive(Debug, Default, Clone)]
pub struct Thread {
    pub id: Uuid,
    pub messages: Vec<String>,
}
