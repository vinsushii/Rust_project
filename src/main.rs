use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufReader};
use std::path::Path;
use std::process;


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
