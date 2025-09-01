use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufReader};
use std::path::Path;
use std::process;

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

#[derive(Debug, Serialize, Deserialize)]
struct Database {
    incidents: HashMap<u32, Incident>,
    last_id: u32,
}

fn main() {
   
}
