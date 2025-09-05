use crate::incidents::{Incident, Status};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufReader, Write},
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Database { 
    incidents: HashMap<u32, Incident>,
    last_id: u32,
}

impl Database {
    pub fn new() -> Self {
        Database {
            incidents: HashMap::new(),
            last_id: 0,
        }
    }

    pub fn load(filename: &str) -> Self {
        if let Ok(file) = File::open(filename) {
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_else(|_| Database::new())
        } else {
            Database::new()
        }
    }

    pub fn save(&self, filename: &str) {
        if let Ok(mut file) = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)
        {
            let data = serde_json::to_string_pretty(&self).unwrap();
            let _ = file.write_all(data.as_bytes());
        }
    }

    pub fn add_incident(
        &mut self,
        datetime: String,
        people_involved: Vec<String>,
        description: String,
    ) {
        self.last_id += 1;
        let incident = Incident {
            id: self.last_id,
            datetime,
            people_involved,
            description,
            status: Status::Pending,
        };
        self.incidents.insert(self.last_id, incident);
        println!("Incident {} added successfully!", self.last_id);
    }

    pub fn list_incidents(&self) -> Vec<&Incident> {
        let mut incidents: Vec<&Incident> = self.incidents.values().collect();
        incidents.sort_by_key(|i| i.id);
        incidents
    }

    pub fn update_status(&mut self, id: u32, new_status: Status) {
        if let Some(incident) = self.incidents.get_mut(&id) {
            incident.status = new_status;
            println!("Incident {} updated to {:?}", id, incident.status);
        } else {
            println!("Incident ID {} not found", id);
        }
    }

    pub fn delete_incident(&mut self, id: u32) {
        if self.incidents.remove(&id).is_some() {
            println!("Incident {} deleted", id)
        } else {
            println!("Incident {} is not deleted.", id);
        }
    }
}
