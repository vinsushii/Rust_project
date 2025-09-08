//! # Charlie Mart Cashier System
//!
//! ## Example Usage
//! ```text
//! Charlie Mart Cashier System
//! 1. View products
//! 2. Add product
//! 3. Update product
//! 4. Delete product
//! 5. Checkout / Create invoice
//! 6. Exit
//! Enter your choice:
//! ```

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

fn menu() {
    
    loop {
        println!("Charlie Mart Incident Management System");
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
