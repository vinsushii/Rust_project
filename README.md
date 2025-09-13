# How to Setup and Download Rust Programming Language

<p>Go to this website:</p>
<a href="https://www.rust-lang.org/tools/install">Install Rust</a>  
<p>Once the file has downloaded, click the installer. It will open a command prompt for setup.</p>  
<p>To ensure Rust is installed correctly, open your command prompt and run:</p>

```bash
rustc --version
cargo --version
```

---

# Charlie Mart Cashier System

<p>A simple <strong>command-line application</strong> written in Rust for managing store products and invoices.<br>
All product data is stored in a local JSON file (<code>products.json</code>) so your inventory <strong>persists between runs</strong>.</p>

---

## Features
<ul>
  <li><strong>View Products</strong> – Display all available products with their stock and prices.</li>
  <li><strong>Add Product</strong> – Add a new item with name, quantity, and price.</li>
  <li><strong>Update Product</strong> – Edit existing product details.</li>
  <li><strong>Delete Product</strong> – Remove an item from inventory.</li>
  <li><strong>Checkout</strong> – Create an invoice, calculate subtotal, tax (12%), and total, then process payment.</li>
  <li><strong>Persistent Storage</strong> – All changes are saved to <code>products.json</code>.</li>
</ul>

---

## Program Flow
<ol>
  <li>The program starts at <code>main.rs</code> and calls the <code>menu</code> function.</li>
  <li>Product data is loaded from <code>products.json</code> (or created if the file doesn’t exist).</li>
  <li>The user is presented with the menu:</li>
</ol>

<pre><code>Charlie Fruits Mart Cashier System
1. View Products
2. Add Products
3. Update Product
4. Delete Product
5. Checkout
6. Exit
Enter an option:
</code></pre>

<ol start="4">
  <li>Depending on your choice:
    <ul>
      <li><strong>View</strong> → Shows the list of products.</li>
      <li><strong>Add</strong> → Inserts a new item with ID, name, quantity, and price.</li>
      <li><strong>Update</strong> → Lets you edit a product’s details.</li>
      <li><strong>Delete</strong> → Removes an item from the inventory.</li>
      <li><strong>Checkout</strong> → Generates an invoice, applies 12% tax, and updates stock levels.</li>
      <li><strong>Exit</strong> → Saves data and closes the program.</li>
    </ul>
  </li>
  <li>All changes are written to <code>products.json</code> after every operation.</li>
</ol>

---

## Installation & Running

### Prerequisites
<p><a href="https://www.rust-lang.org/tools/install">Rust</a> (latest stable version recommended)</p>

### Steps
<pre><code># Clone this repository
git clone https://github.com/yourusername/charlie-mart-cashier.git
cd charlie-mart-cashier

# Run the program
cargo run
</code></pre>

---

## Example Usage

<pre><code>Charlie Fruits Mart Cashier System
1. View Products
2. Add Products
3. Update Product
4. Delete Product
5. Checkout
6. Exit
Enter an option: 2
Enter product name: Apple
Enter quantity: 20
Enter price: 15
Product added successfully!
Press Enter to continue...
</code></pre>

<p>Checkout example:</p>

<pre><code>--- Checkout ---
Enter product ID to add to invoice (or 'done'): 1
Enter quantity for Apple: 5
Added: Apple x5 = ₱75.00
-------------------------
SUBTOTAL: ₱75.00
TAX (12%): ₱9.00
TOTAL: ₱84.00
Enter payment: 100
Payment successful. Change: ₱16.00
</code></pre>

---

## Project Structure
<pre><code>src/
 ├── main.rs       # CLI menu and program entry point
 └── products.rs   # Product struct and JSON handling
products.json      # Persistent storage for inventory
</code></pre>

---

### Developer Documentation

<p>You can generate detailed documentation for the project with:</p>

```bash
cargo doc --open
```
