use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Status {
    Pending,
    InProgress,
    Resolved,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Incident {
    id: u32,
    datetime: String,
    people_involved: Vec<String>,
    description: String,
    pub(crate) status: Status,
}

