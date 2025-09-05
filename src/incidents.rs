use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Status {
    Pending,
    InProgress,
    Resolved,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Incident {
    pub(crate) id: u32,
    pub(crate) datetime: String,
    pub(crate) people_involved: Vec<String>,
    pub(crate) description: String,
    pub(crate) status: Status,
}
