use crate::incidents::{Incident, Status};
use std::{collections::HashMap, fs::{File, OpenOptions}, io::{BufReader, Write}};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
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

  pub  fn update_status(&mut self, id: u32, new_status: Status) {
        if let Some(incident) = self.incidents.get_mut(&id) {
            incident.status = new_status;
            println!("Incident {} updated to {:?}", id, incident.status);
        } else {
            println!("Incident ID {} not found", id);
        }
    }
    pub fn delete_incident(&mut self, id: u32) {
        if self.incidents.remove(&id).is_some(){
            println!("Incident {} deleted", id)
        }else {
            print!("Incident {} is not deleted.", id);
        }
    }
}
