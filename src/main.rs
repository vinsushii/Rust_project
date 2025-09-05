//! # Barangay Incident Report Logger Management System
//!
//! This program is a simple **command-line tool** for managing incidents.
//!
//! ## Program Flow
//! 1. When the program starts, it calls [`menu`].
//! 2. [`menu`] loads existing incidents from `incidents.json` (or starts fresh if the file doesn’t exist).
//! 3. The user is shown a menu of options:
//!    - Add a new incident
//!    - View all incidents
//!    - Update an incident’s status
//!    - Delete an incident
//!    - Exit (saving changes)
//! 4. Based on the user’s input, the appropriate method from [`Database`](crate::database::Database) is called.
//! 5. All changes are saved to `incidents.json`, so data persists between runs.
//!
//! ## Example Usage
//! ```text
//! Incident Management System
//! 1. Add Incident
//! 2. View Incidents
//! 3. Update Incident Status
//! 4. Delete Incident
//! 5. Exit
//! Enter your choice:
//! ```

pub(crate) mod database;
mod incidents;

use std::io::{self, Write};
use std::process;
use crate::incidents::Status;

/// Pauses program execution until the user presses Enter.
///
/// This is used to give the user time to read program output
/// before returning to the menu.
fn pause() {
    let mut input = String::new();
    println!("Press Enter to continue...");
    let _ = io::stdin().read_line(&mut input);
}

/// Displays the main menu loop for the Incident Management System.
///
/// Handles user input and routes actions to the appropriate database functions:
/// - **1**: Add a new incident
/// - **2**: View all incidents
/// - **3**: Update the status of an incident
/// - **4**: Delete an incident
/// - **5**: Save data and exit
fn menu() {
    // Load the database from the JSON file (or create new if not found)
    let mut db: crate::database::Database = crate::database::Database::load("incidents.json");

    loop {
        println!("Barangay Incident Report Logger");
        println!("1. Add Incident");
        println!("2. View Incidents");
        println!("3. Update Incident Status");
        println!("4. Delete Incident");
        println!("5. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            // Add Incident
            "1" => {
                let mut datetime = String::new();
                let mut description = String::new();
                let mut people = String::new();

                print!("Enter datetime: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut datetime).unwrap();

                print!("Enter people involved (comma-separated): ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut people).unwrap();
                let people_involved: Vec<String> = people
                    .trim()
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();

                print!("Enter description: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut description).unwrap();

                db.add_incident(
                    datetime.trim().to_string(),
                    people_involved,
                    description.trim().to_string(),
                );
                db.save("incidents.json");
                pause();
            }

            // View Incidents
            "2" => {
                let incidents = db.list_incidents();
                if incidents.is_empty() {
                    println!("No incidents found.");
                } else {
                    println!("--- Incident List ---");
                    for i in incidents {
                        println!(
                            "ID: {}\nDateTime: {}\nPeople: {}\nDescription: {}\nStatus: {:?}\n",
                            i.id,
                            i.datetime,
                            i.people_involved.join(", "),
                            i.description,
                            i.status
                        );
                    }
                }
                pause();
            }

            // Update Incident Status
            "3" => {
                let mut id_str = String::new();
                let mut status_str = String::new();

                print!("Enter Incident ID to update: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut id_str).unwrap();

                print!("Enter new status (Pending/InProgress/Resolved): ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut status_str).unwrap();

                let id = id_str.trim().parse::<u32>().unwrap_or(0);
                let status = match status_str.trim().to_lowercase().as_str() {
                    "pending" => Status::Pending,
                    "inprogress" => Status::InProgress,
                    "resolved" => Status::Resolved,
                    _ => {
                        println!("Invalid status entered");
                        continue;
                    }
                };
                db.update_status(id, status);
                db.save("incidents.json");
                pause();
            }

            // Delete Incident
            "4" => {
                let mut id_str = String::new();
                print!("Enter Incident ID for Deletion: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut id_str).unwrap();

                let id = id_str.trim().parse::<u32>().unwrap_or(0);
                db.delete_incident(id);
                db.save("incidents.json");
                pause();
            }

            // Exit Program
            "5" => {
                db.save("incidents.json");
                println!("Exiting...");
                process::exit(0);
            }

            // Invalid Input
            _ => println!("Invalid choice, please try again."),
        }
    }
}

/// Program entry point.
///
/// Runs the [`menu`] function, which provides the CLI loop.
fn main() {
    menu();
}
