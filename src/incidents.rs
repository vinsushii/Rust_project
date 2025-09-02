use std::collections::HashMap;
use serde::{Serialize, Deserialize};
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

