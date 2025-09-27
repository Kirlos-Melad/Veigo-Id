# veigo-id

**Veigo ID** is a **thread-safe, configurable 128-bit unique ID generator** designed for distributed systems, inspired by Twitter's Snowflake. It provides collision-resistant IDs with customizable bit layouts for timestamp, context, and counter, and supports pluggable state backends like in-memory, Redis, or databases.

---

## Features

- **128-bit IDs** with configurable layout (timestamp/context/counter)  
- **Thread-safe** by design  
- **Pluggable state backend**, allowing distributed storage (memory, Redis, SQL, etc.)  
- **Lightweight**, minimal dependencies  
- **Decode IDs** to get timestamp, context, and counter components  

---

## Installation

Add as a Git dependency:

```toml
[dependencies]
veigo-id = { git = "https://github.com/Kirlos-Melad/Veigo-Id.git" }
```
---

## Example Usage

```rust
use std::sync::Arc;
use veigo_id::{MemoryState, VeigoIdGenerator};

fn main() {
    let backend = Arc::new(MemoryState::new());
    let generator = VeigoIdGenerator::new(None, backend).unwrap();

    // Generate a new ID with a context
    let context = 42u128;
    let id = generator.generate(context).unwrap();

    // Decode the ID
    let parts = generator.decode(id);
    println!("Timestamp: {}", parts.timestamp);
    println!("Context: {}", parts.context);
    println!("Counter: {}", parts.counter);
}
```

