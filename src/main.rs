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
fn save_products(file: &str, products: &Vec<Product>) {
    let data = serde_json::to_string_pretty(products).unwrap();
    std::fs::write(file, data).unwrap();
}

fn menu() {
    let file = "products.json";
    let mut products = load_products(file);
    const  TAX_RATE: f64 = 0.12; // Constant tax rate

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
                if products.is_empty() {
                    println!("No products available.");
                } else {
                    println!("\n--- Product List ---");
                    for product in &products {
                        println!(
                            "ID: {} | Name: {} | Quantity: {} | Price: {}",
                            product.id, product.product, product.quantity, product.price
                        );
                    }
                }
                pause();
            }
            "2" => {
                let mut name = String::new();
                let mut quantity = String::new();
                let mut price = String::new();
                println!("\n--- Add New Product ---");
                print!("Enter product name: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut name).unwrap();
                print!("Enter quantity: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut quantity).unwrap();
                print!("Enter price: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut price).unwrap();
                let id = products.last().map_or(1, |p| p.id + 1);
                let product = Product {
                    id,
                    product: name.trim().to_string(),
                    quantity: quantity.trim().parse().unwrap_or(0.0),
                    price: price.trim().parse().unwrap_or(0.0),
                };
                products.push(product);
                save_products(file, &products);
                println!("Product added successfully.");
                
                pause();
            }
            "3" => {
                if products.is_empty() {
                    println!("No products available to update.");
                } else {
                    println!("\n--- Product List ---");
                    for product in &products {
                        println!(
                            "ID: {} | Name: {} | Quantity: {} | Price: {}",
                            product.id, product.product, product.quantity, product.price
                        );
                    }

                    print!("\nEnter the Product ID to update: ");
                    io::stdout().flush().unwrap();

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim();

                    if let Ok(id) = input.parse::<u32>() {
                        if let Some(product) = products.iter_mut().find(|p| p.id == id) {
                            let mut name = String::new();
                            let mut quantity = String::new();
                            let mut price = String::new();

                            print!("Enter new product name (current: {}): ", product.product);
                            io::stdout().flush().unwrap();
                            io::stdin().read_line(&mut name).unwrap();
                            print!("Enter new quantity (current: {}): ", product.quantity);
                            io::stdout().flush().unwrap();
                            io::stdin().read_line(&mut quantity).unwrap();
                            print!("Enter new price (current: {}): ", product.price);
                            io::stdout().flush().unwrap();
                            io::stdin().read_line(&mut price).unwrap();

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
            "5" => {
                println!("\n--- Checkout ---");
                let mut subtotal = 0.0; // running subtotal before tax

                loop {
                    // Ask the user for product ID
                    let mut id_str = String::new();
                    println!("Enter product ID to add to invoice (or 'done'):");
                    io::stdin().read_line(&mut id_str).unwrap();
                    let id_str = id_str.trim().to_lowercase(); // clean up input

                    // If user types "done", exit the loop
                    if id_str.eq_ignore_ascii_case("done") {
                        break;
                    }

                    // Convert product ID from string to number (default 0 if invalid)
                    let id: u32 = id_str.parse().unwrap_or(0);

                    // Try to find the product with that ID
                    if let Some(products) = products.iter().find(|p| p.id == id) {
                        // If product exists, ask for quantity
                        let mut qty_str = String::new();
                        println!("Enter quantity for {}:", products.product);
                        io::stdin().read_line(&mut qty_str).unwrap();
                        let qty: u32 = qty_str.trim().parse().unwrap_or(1);

                        // Break product into fields (ignore id since we don’t need it)
                        let Products { id: _, product, price } = products.clone();

                        // Compute line total = price × quantity
                        let line_total = price * qty as f64;
                        subtotal += line_total;

                        println!("Added: {} x{} = ₱{:.2}", product, qty, line_total);
                    } else {
                        // If no product is found
                        println!("Product not found.");
                    }
                }

                // After all items, compute tax and total
                let tax = subtotal * TAX_RATE;
                let total = subtotal + tax;

                println!("-------------------------");
                println!("SUBTOTAL:  ₱{:.2}", subtotal);
                println!("TAX (12%): ₱{:.2}", tax);
                println!("TOTAL:     ₱{:.2}", total);

                // Ask for payment
                let mut payment = String::new();
                println!("Enter payment:");
                io::stdin().read_line(&mut payment).unwrap();
                let payment: f64 = payment.trim().parse().unwrap_or(0.0);

                // Check if payment is enough
                if payment >= total {
                    println!("Payment successful. Change: ₱{:.2}", payment - total);
                } else {
                    println!("Not enough payment. Short by ₱{:.2}", total - payment);
                }

                // Pause before going back to menu
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
