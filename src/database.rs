//! This module manages all the **incident records**.
//!
//! It acts like a small database that stores [`Incident`](crate::incidents::Incident)
//! objects in memory and saves them to a JSON file.
//!
//! ## How It Fits in the Flow
//! - When the program starts, [`Database::load`] loads existing data.
//! - When the user adds an incident, [`Database::add_incident`] is called.
//! - When listing incidents, [`Database::list_incidents`] provides them in order.
//! - When updating, [`Database::update_status`] changes the status.
//! - When deleting, [`Database::delete_incident`] removes an incident.
//! - Before exiting, [`Database::save`] writes everything back to disk.

use crate::incidents::{Incident, Status};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufReader, Write},
};
use serde::{Serialize, Deserialize};

/// Represents the incident database.
///
/// Stores all [`Incident`](crate::incidents::Incident) records
/// and handles persistence to a JSON file.
#[derive(Debug, Serialize, Deserialize)]
pub struct Database { 
    /// Internal storage of incidents, mapped by their unique ID.
    incidents: HashMap<u32, Incident>,

    /// The last assigned incident ID (auto-increment counter).
    last_id: u32,
}

impl Database {
    /// Creates a new, empty database.
    pub fn new() -> Self {
        Database {
            incidents: HashMap::new(),
            last_id: 0,
        }
    }

    /// Loads a database from a JSON file.
    ///
    /// If the file does not exist or fails to parse, returns an empty database.
    ///
    /// # Arguments
    ///
    /// * `filename` - The path to the JSON file.
    pub fn load(filename: &str) -> Self {
        if let Ok(file) = File::open(filename) {
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_else(|_| Database::new())
        } else {
            Database::new()
        }
    }

    /// Saves the database to a JSON file.
    ///
    /// Overwrites the file if it already exists.
    ///
    /// # Arguments
    ///
    /// * `filename` - The path to the JSON file.
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

    /// Adds a new incident to the database.
    ///
    /// Automatically assigns a unique ID and sets its status to [`Status::Pending`].
    ///
    /// # Arguments
    ///
    /// * `datetime` - The datetime of the incident.
    /// * `people_involved` - A list of people involved in the incident.
    /// * `description` - A text description of the incident.
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

    /// Returns all incidents in the database, sorted by ID.
    pub fn list_incidents(&self) -> Vec<&Incident> {
        let mut incidents: Vec<&Incident> = self.incidents.values().collect();
        incidents.sort_by_key(|i| i.id);
        incidents
    }

    /// Updates the status of an incident.
    ///
    /// If the incident ID is not found, a warning message is printed.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique ID of the incident.
    /// * `new_status` - The new status to apply.
    pub fn update_status(&mut self, id: u32, new_status: Status) {
        if let Some(incident) = self.incidents.get_mut(&id) {
            incident.status = new_status;
            println!("Incident {} updated to {:?}", id, incident.status);
        } else {
            println!("Incident ID {} not found", id);
        }
    }

    /// Deletes an incident by ID.
    ///
    /// Prints a message indicating whether deletion was successful.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique ID of the incident to delete.
    pub fn delete_incident(&mut self, id: u32) {
        if self.incidents.remove(&id).is_some() {
            println!("Incident {} deleted", id)
        } else {
            println!("Incident {} is not deleted.", id);
        }
    }
}
