# How to Setup and Download Rust Programming Language

<p>Go to this website:</p>
<a href="https://www.rust-lang.org/tools/install">Install Rust</a>
<p>Once the file has downloaded, click the installer. It will open a command prompt for setup.</p>
<p>To ensure Rust is installed correctly, open your command prompt and run:</p>

```bash
rustc --version
cargo --version
```

<h1>Barangay Incident Report Logger Management System</h1>

<p>A simple <strong>command-line tool</strong> written in Rust for managing incidents.<br>
Incidents are stored in a local JSON file (<code>incidents.json</code>) so that your data <strong>persists between runs</strong>.</p>

<hr>

<h2>Features</h2>
<ul>
  <li><strong>Add Incident</strong> – Record a new incident with datetime, people involved, and a description.</li>
  <li><strong>View Incidents</strong> – See a list of all recorded incidents with details and status.</li>
  <li><strong>Update Incident Status</strong> – Change an incident’s status (<code>Pending</code>, <code>InProgress</code>, <code>Resolved</code>).</li>
  <li><strong>Delete Incident</strong> – Remove an incident by ID.</li>
  <li><strong>Persistent Storage</strong> – Data is saved to <code>incidents.json</code> and automatically reloaded.</li>
</ul>

<hr>

<h2>Program Flow</h2>
<ol>
  <li>The program starts in <code>main.rs</code> and calls the <code>menu</code> function.</li>
  <li>The <code>Database</code> is loaded from <code>incidents.json</code> (or created fresh if not found).</li>
  <li>The user is shown a menu:</li>
</ol>

<pre><code>Incident Management System
1. Add Incident
2. View Incidents
3. Update Incident Status
4. Delete Incident
5. Exit
Enter your choice:
</code></pre>

<ol start="4">
  <li>
    Based on input:
    <ul>
      <li><strong>Add</strong> → Creates a new <code>Incident</code> (default status: <code>Pending</code>)</li>
      <li><strong>View</strong> → Lists all incidents</li>
      <li><strong>Update</strong> → Changes an incident’s <code>Status</code></li>
      <li><strong>Delete</strong> → Removes an incident</li>
      <li><strong>Exit</strong> → Saves the database and terminates</li>
    </ul>
  </li>
  <li>Data is saved to <code>incidents.json</code> after every operation and on exit.</li>
</ol>

<hr>

<h2>Installation &amp; Running</h2>

<h3>Prerequisites</h3>
<p><a href="https://www.rust-lang.org/tools/install">Rust</a> (latest stable version recommended)</p>

<h3>Steps</h3>
<pre><code># Clone this repository
git clone https://github.com/yourusername/incident-management-system.git
cd incident-management-system

# Run the program
cargo run
</code></pre>

<hr>

<h2>Example Usage</h2>

<pre><code>Barangay Incident Report Logger
1. Add Incident
2. View Incidents
3. Update Incident Status
4. Delete Incident
5. Exit
Enter your choice: 1
Enter datetime: 2025-09-06 14:00
Enter people involved: Alice, Bob
Enter description: System outage in server room
Incident 1 added successfully!
Press Enter to continue...
</code></pre>

<p>Viewing incidents:</p>

<pre><code>--- Incident List ---
ID: 1
DateTime: 2025-09-06 14:00
People: Alice, Bob
Description: System outage in server room
Status: Pending
</code></pre>

<hr>

<h2>Project Structure</h2>
<pre><code>src/
 ├── main.rs       # CLI menu and program entry point
 ├── incidents.rs  # Incident struct and Status enum
 └── database.rs   # Database struct for storing and managing incidents
</code></pre>

<hr>

<h2>Future Improvements</h2>
<ul>
  <li>Input validation for datetime format</li>
  <li>Search/filter incidents by keyword</li>
  <li>Export to CSV for reports</li>
  <li>More user-friendly CLI with colors</li>
</ul>

<hr>

<p>You can also generate developer documentation using:</p>

<pre><code>cargo doc --open
</code></pre>
