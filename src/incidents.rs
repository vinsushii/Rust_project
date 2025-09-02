
#[derive(Debug, Serialize, Deserialize, Clone)]
enum Status {
    Pending,
    InProgress,
    Resolved,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Incident {
    id: u32,
    datetime: String,
    people_involved: Vec<String>,
    description: String,
    status: Status,
}

