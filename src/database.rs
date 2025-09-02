use crate::incidents::{Incident, Status};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufReader};
use std::path::Path;
use std::process;

#[derive(Debug, Serialize, Deserialize)]
struct Database {
    incidents: HashMap<u32, Incident>,
    last_id: u32,
}

impl Database {
    fn new() -> Self {
        Database {
            incidents: HashMap::new(),
            last_id: 0,
        }
    }

    fn update_status(&mut self, id: u32, new_status: Status) {
        if let Some(incident) = self.incidents.get_mut(&id) {
            incident.status = new_status;
            println!("Incident {} updated to {:?}", id, incident.status);
        } else {
            println!("Incident ID {} not found", id);
        }
    }
}
