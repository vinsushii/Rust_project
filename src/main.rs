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

fn pause() {
    let mut input = String::new();
    println!("Press Enter to continue...");
    let _ = io::stdin().read_line(&mut input);
}

fn menu() {
    let db = Database::load("incidents.json");
    loop {
        println!("Incident Management System");
        println!("1. Add Incident");
        println!("2. View Incidents");
        println!("3. Update Incident Status");
        println!("4. Delete Incident");
        println!("5. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choic = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => {
                
            }
            "2" => {
                
            }
            "3" => {
                
            }
            "4" => {
                let mut id_str = String::new();
                let mut status_str = String::new();

                print!("Enter Incident ID to update: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut id_str).unwrap();

                print!("Enter new status (Pending/InProgress/Resolved): ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut status_str).unwrap();

                let id = id_str.trim().parse()::<u32>().unwrap_or(0);
                let status = match status_str.trim().to_lowercase().as_str() {
                    "pending" => Status::Pending,
                    "inprogress" => Status::InProgress,
                    "resolved" => Status::Resolved,
                    _ => {
                        println!("Invalid status entered>");
                        continue;
                    }
                };
                db.update_status(id, status);
                db.save("incidents.json");
            }
            "5" => {
                db.save("incidents.json");
                println!("Exiting...");
                process::exit(0);
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn main() {
   menu();
}
