# CGDB - Contextual Graph Database

CGDB is a Contextual Graph Database designed to handle complex data relationships with dynamic schema evolution and context-aware querying. This database system is written in Rust and aims to provide an efficient, scalable, and flexible solution for managing interconnected data.

### Features
- Contextual Nodes and Edges
- Dynamic Schema Evolution
- Multi-Dimensional Relationships
- Contextual Queries
- Temporal and Spatial Awareness
- Adaptive Indexing
- Intelligent Data Sharding
- Semantic Understanding

### Table of Contents
- [Installation](#installation)
- [Compiling](#compiling)
- [Usage](#usage)
  - [Initializing the Database](#initializing-the-database)
  - [Adding Nodes and Edges](#adding-nodes-and-edges)
  - [Querying Data](#querying-data)
- [Example](#example)
- [Running as a Daemon](#running-as-a-daemon)
  - [Setting up the HTTP Server](#setting-up-the-http-server)
  - [Running with nohup](#running-with-nohup)
  - [Using systemd](#using-systemd)
- [License](#license)

### Installation
To install CGDB, you need to have Rust and Cargo installed on your machine. Follow these steps to set up the project:

1. **Clone the Repository**
   ```sh
   git clone https://github.com/yourusername/cgdb.git
   cd cgdb
   ```

2. **Build the Project**
   ```sh
   cargo build
   ```

3. **Run the Project**
   ```sh
   cargo run
   ```

### Compiling
To compile the project, simply use Cargo:
```sh
cargo build --release
```
This will compile the project in release mode, optimizing for performance.

### Usage

#### Initializing the Database
Create an instance of the `ContextService` to manage contexts, nodes, and edges:
```rust
use services::context_service::ContextService;

let mut context_service = ContextService::new();
```

#### Adding Nodes and Edges
Add contexts, nodes, and edges to the database:
```rust
use models::context::Context;
use database::node::Node;
use database::edge::Edge;

// Create a new context
let context = Context::new(1, "Sample Context");
context_service.add_context(context);

// Add nodes to the context
context_service.add_node_to_context(1, 101, "Node 1");
context_service.add_node_to_context(1, 102, "Node 2");

// Add an edge between nodes
context_service.add_edge_to_context(1, 101, 102, "Edge from Node 1 to Node 2");
```

#### Querying Data
Retrieve and query data from the database:
```rust
// Retrieve and display the context
if let Some(retrieved_context) = context_service.get_context(1) {
    println!("Context: {:?}", retrieved_context);
}

// Retrieve and display a node
if let Some(node) = context_service.schema.get_node(101) {
    println!("Node: {:?}", node);
}

// Retrieve and display an edge
if let Some(edge) = context_service.schema.get_edge(101, 102) {
    println!("Edge: {:?}", edge);
}

// Query nodes by property
if let Some(node) = context_service.schema.nodes.get_mut(&101) {
    node.add_property("key", "value");
}
let nodes_with_property = context_service.schema.query_nodes_by_property("key", "value");
for node in nodes_with_property {
    println!("Node with property: {:?}", node);
}
```

### Example
Here's an example of a full program using CGDB:
```rust
mod database;
mod models;
mod services;
mod api;
mod config;
mod utils;

use models::context::Context;
use database::node::Node;
use database::edge::Edge;
use services::context_service::ContextService;

fn main() {
    // Initialize the context service
    let mut context_service = ContextService::new();

    // Create a new context
    let context = Context::new(1, "Sample Context");
    context_service.add_context(context);

    // Add nodes to the context
    context_service.add_node_to_context(1, 101, "Node 1");
    context_service.add_node_to_context(1, 102, "Node 2");

    // Add an edge between nodes
    context_service.add_edge_to_context(1, 101, 102, "Edge from Node 1 to Node 2");

    // Retrieve and display the context
    if let Some(retrieved_context) = context_service.get_context(1) {
        println!("Context: {:?}", retrieved_context);
    }

    // Retrieve and display a node
    if let Some(node) = context_service.schema.get_node(101) {
        println!("Node: {:?}", node);
    }

    // Retrieve and display an edge
    if let Some(edge) = context_service.schema.get_edge(101, 102) {
        println!("Edge: {:?}", edge);
    }

    // Query nodes by property (for demonstration, properties need to be added to nodes)
    // Example of adding a property to a node
    if let Some(node) = context_service.schema.nodes.get_mut(&101) {
        node.add_property("key", "value");
    }

    let nodes_with_property = context_service.schema.query_nodes_by_property("key", "value");
    for node in nodes_with_property {
        println!("Node with property: {:?}", node);
    }
}
```

### Running as a Daemon

#### Running with `nohup`
1. **Compile the Project**:
   ```sh
   cargo build --release
   ```

2. **Run the Application in the Background**:
   ```sh
   nohup ./target/release/cgdb &
   ```

#### Using `systemd`
For a more robust solution, create a `systemd` service file to manage your daemon.

1. **Create the Service File**:
   ```sh
   sudo nano /etc/systemd/system/cgdb.service
   ```

2. **Add the Following Content**:
   ```ini
   [Unit]
   Description=CGDB Service
   After=network.target

   [Service]
   ExecStart=/path/to/your/cgdb/target/release/cgdb
   Restart=on-failure
   User=youruser
   Group=yourgroup

   [Install]
   WantedBy=multi-user.target
   ```

3. **Enable and Start the Service**:
   ```sh
   sudo systemctl enable cgdb
   sudo systemctl start cgdb
   ```

### Connecting to the Daemon
You can connect to your running daemon using any HTTP client. Here are some examples using `curl`:

1. **Add a Context**:
   ```sh
   curl http://localhost:3000/add_context
   ```

2. **Get a Context**:
   ```sh
   curl http://localhost:3000/get_context
   ```
