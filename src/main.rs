//! # Charlie Mart Cashier System
//!
//! A simple command-line cashier application written in Rust.
//!
//! ## Features
//! - View, add, update, and delete products
//! - Save inventory to a JSON file
//! - Checkout with tax computation
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

mod products; // Module containing the `Product` struct
use products::Product;
use std::fs;
use std::io::{self, Write};

/// Pauses program execution until the user presses **Enter**.
///
/// This is used so the user can read the output before returning to the menu.
fn pause() {
    let mut input = String::new();
    println!("Press Enter to continue...");
    let _ = io::stdin().read_line(&mut input); // Wait for user input
}

/// Loads product data from a JSON file.
/// Returns a vector of `Product`.
fn load_products(file: &str) -> Vec<Product> {
    // Read the entire file; if it fails, return an empty array string "[]"
    let data = fs::read_to_string(file).unwrap_or_else(|_| "[]".to_string());
    // Parse JSON into Vec<Product>; if it fails, return an empty Vec
    serde_json::from_str(&data).unwrap_or_else(|_| vec![])
}

/// Saves a list of products into a JSON file.
fn save_products(file: &str, products: &Vec<Product>) {
    // Convert Vec<Product> into a pretty JSON string
    let data = serde_json::to_string_pretty(products).unwrap();
    // Write the JSON string to the file
    std::fs::write(file, data).unwrap();
}

/// Displays the menu, handles user input, and performs CRUD/checkout operations.
fn menu() {
    let file = "products.json"; // Path to our inventory file
    let mut products = load_products(file); // Load products into memory

    const TAX_RATE: f64 = 0.12; // Tax rate for checkout

    loop {
        // Display menu options
        println!("Charlie Fruits Mart Cashier System");
        println!("1. View Products");
        println!("2. Add Products");
        println!("3. Update Product");
        println!("4. Delete Product");
        println!("5. Checkout");
        println!("6. Exit");
        print!("Enter an option: ");
        io::stdout().flush().unwrap(); // Flush so prompt shows before input

        // Read the user's choice
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            // VIEW PRODUCTS
            "1" => {
                if products.is_empty() {
                    println!("No products available.");
                } else {
                    println!("\n--- Product List ---");
                    // Iterate over all products and print their details
                    for product in &products {
                        println!(
                            "ID: {} | Name: {} | Quantity: {} | Price: {}",
                            product.id, product.product, product.quantity, product.price
                        );
                    }
                }
                pause(); // Wait for user before returning to menu
            }
            // ADD PRODUCT
           "2" => {
                println!("\n--- Add New Product ---");

                // Product Name
                let name = loop {
                    let mut input = String::new();
                    print!("Enter product name: ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut input).unwrap();

                    let trimmed = input.trim();
                    if !trimmed.is_empty() {
                        break trimmed.to_string();
                    } else {
                        println!("Product name cannot be empty. Please try again.");
                    }
                };

                // Quantity (must be a valid f64)
                let quantity = loop {
                    let mut input = String::new();
                    print!("Enter quantity: ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut input).unwrap();

                    match input.trim().parse::<f64>() {
                        Ok(q) if q >= 0.0 => break q,
                        _ => println!("Invalid quantity. Please enter a valid number."),
                    }
                };

                // Price (must be a valid f64)
                let price = loop {
                    let mut input = String::new();
                    print!("Enter price: ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut input).unwrap();

                    match input.trim().parse::<f64>() {
                        Ok(p) if p >= 0.0 => break p,
                        _ => println!("Invalid price. Please enter a valid number."),
                    }
                };

                // Generate next product ID based on last ID in list
                let id = products.last().map_or(1, |p| p.id + 1);

                // Create new product
                let product = Product {
                    id,
                    product: name,
                    quantity,
                    price,
                };

                // Add to vector and save
                products.push(product);
                save_products(file, &products);
                println!("✅ Product added successfully.");
                pause();
            }

            // UPDATE PRODUCT
            "3" => {
                if products.is_empty() {
                    println!("No products available to update.");
                } else {
                    // Display all products for reference
                    println!("\n--- Product List ---");
                    for product in &products {
                        println!(
                            "ID: {} | Name: {} | Quantity: {} | Price: {}",
                            product.id, product.product, product.quantity, product.price
                        );
                    }

                    print!("\nEnter the Product ID to update: ");
                    io::stdout().flush().unwrap();

                    // Read the ID
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim();

                    if let Ok(id) = input.parse::<u32>() {
                        // Search for the product by ID
                        if let Some(product) = products.iter_mut().find(|p| p.id == id) {
                            let mut name = String::new();
                            let mut quantity = String::new();
                            let mut price = String::new();

                            // Prompt for new details, showing current values
                            print!("Enter new product name (current: {}): ", product.product);
                            io::stdout().flush().unwrap();
                            io::stdin().read_line(&mut name).unwrap();

                            print!("Enter new quantity (current: {}): ", product.quantity);
                            io::stdout().flush().unwrap();
                            io::stdin().read_line(&mut quantity).unwrap();

                            print!("Enter new price (current: {}): ", product.price);
                            io::stdout().flush().unwrap();
                            io::stdin().read_line(&mut price).unwrap();

                            // Update fields if not empty
                            if !name.trim().is_empty() {
                                product.product = name.trim().to_string();
                            }
                            if !quantity.trim().is_empty() {
                                if let Ok(qty) = quantity.trim().parse() {
                                    product.quantity = qty;
                                }
                            }
                            if let Ok(prc) = price.trim().parse() {
                                product.price = prc;
                            }

                            save_products(file, &products);
                            println!("Product updated successfully.");
                        } else {
                            println!("No product found with ID {}", id);
                        }
                    } else {
                        println!("Invalid ID input.");
                    }
                }
                pause();
            }
            // DELETE PRODUCT
            "4" => {
                if products.is_empty() {
                    println!("No products available to delete.");
                } else {
                    println!("\n--- Product List ---");
                    for product in &products {
                        println!(
                            "ID: {} | Name: {} | Quantity: {} | Price: {}",
                            product.id, product.product, product.quantity, product.price
                        );
                    }

                    print!("\nEnter the Product ID to delete: ");
                    io::stdout().flush().unwrap();

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim();

                    if let Ok(id) = input.parse::<u32>() {
                        // Find the product position in vector
                        if let Some(pos) = products.iter().position(|p| p.id == id) {
                            let removed = products.remove(pos);
                            save_products(file, &products);
                            println!("Deleted product: {}", removed.product);
                        } else {
                            println!("No product found with ID {}", id);
                        }
                    } else {
                        println!("Invalid ID input.");
                    }
                }
                pause();
            }
            // CHECKOUT
            "5" => {
                println!("\n--- Checkout ---");
                let mut subtotal = 0.0;

                loop {
                    // Ask for product ID or "done"
                    print!("Enter product ID to add to invoice (or 'done'): ");
                    io::stdout().flush().unwrap();

                    let mut id_str = String::new();
                    io::stdin().read_line(&mut id_str).unwrap();
                    let id_str = id_str.trim();

                    if id_str.eq_ignore_ascii_case("done") {
                        break; // Finish checkout
                    }

                    // Parse the entered ID
                    let id: u32 = match id_str.parse() {
                        Ok(v) => v,
                        Err(_) => {
                            println!("Invalid ID. Try again.");
                            continue;
                        }
                    };

                    // Find the product
                    if let Some(prod) = products.iter_mut().find(|p| p.id == id) {
                        // Ask for quantity
                        print!("Enter quantity for {}: ", prod.product);
                        io::stdout().flush().unwrap();

                        let mut qty_str = String::new();
                        io::stdin().read_line(&mut qty_str).unwrap();
                        let qty: f64 = qty_str.trim().parse().unwrap_or(1.0);

                        // Validate stock
                        if qty > prod.quantity {
                            println!("Not enough stock! Available: {}", prod.quantity);
                            continue;
                        }

                        // Compute line total and update subtotal
                        let line_total = prod.price * qty;
                        subtotal += line_total;

                        // Deduct purchased quantity from stock
                        prod.quantity -= qty;

                        println!("Added: {} x{} = ₱{:.2}", prod.product, qty, line_total);
                    } else {
                        println!("Product not found.");
                    }
                }

                // Compute tax and total
                let tax = subtotal * TAX_RATE;
                let total = subtotal + tax;
                println!("-------------------------");
                println!("SUBTOTAL:  ₱{:.2}", subtotal);
                println!("TAX (12%): ₱{:.2}", tax);
                println!("TOTAL:     ₱{:.2}", total);

                // Ask for payment
                let mut payment = String::new();
                print!("Enter payment: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut payment).unwrap();
                let payment: f64 = payment.trim().parse().unwrap_or(0.0);

                // Check if enough payment
                if payment >= total {
                    println!("Payment successful. Change: ₱{:.2}", payment - total);
                } else {
                    println!("Not enough payment. Short by ₱{:.2}", total - payment);
                }

                // Save updated quantities after checkout
                save_products(file, &products);
                pause();
            }
            // EXIT
            "6" => {
                println!("Exiting...");
                break;
            }
            // INVALID OPTION
            _ => {
                println!("Invalid option, please try again.");
                pause();
            }
        }
    }
}

/// Program entry point.
///
/// Starts the application by calling [`menu`].
fn main() {
    menu();
}