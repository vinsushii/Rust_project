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

mod products;
use products::Product;
use std::fs;
use std::io::{self, Write};

/// Pauses program execution until the user presses Enter.
///
/// This is used to give the user time to read program output
/// before returning to the menu.
fn pause() {
    let mut input = String::new();
    println!("Press Enter to continue...");
    let _ = io::stdin().read_line(&mut input);
}

fn load_products(file: &str) -> Vec<Product> {
    let data = fs::read_to_string(file).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| vec![])
}

fn menu() {
    let file = "products.json";
    let mut products = load_products(file);
    loop {
        println!("Charlie Fruits Mart Cashier System");
        println!("1. View Products");
        println!("2. Add Products");
        println!("3. Update Product");
        println!("4. Delete Product");
        println!("5. Checkout / Create Invoice");
        println!("6. Exit");
        print!("Enter an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                
                pause();
            }
            "2" => {
                
                pause();
            }
            "3" => {
                
                pause();
            }
            "4" => {
                
                pause();
            }
            "5" => {
                println!("Checking out / Creating invoice...");
                
                pause();
            }
            "6" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid option, please try again.");
                pause();
            }
        }
        
    }
}

/// Program entry point.
///
/// Runs the [`menu`] function, which provides the CLI loop.
fn main() {
    menu();
}
