# Objects, Classes & Data Containers / Structures

- Objects and Data Structures are different.

- Objects:
  - Hide their data behind abstractions and expose functions that operate on that data.
  - Contains the business logic in the form of Object-Oriented Programming (OOP).
  - They prefer abstraction over concretions
- Data structure:
  - Expose their data and have no meaningful functions.
  - Contains data with no logic in the form of Procedural Programming.
  - Store and transport data.
  - It only uses concretions.

```Rust
// Data structure
struct Point {
    x: i32,
    y: i32,
}
// Object
impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64
    }
}
```

- It's important to understand the difference between objects and data structures because it helps to understand the difference between procedural programming and object-oriented programming and it helps to understand the difference between good and bad code.

```Rust
pub struct Database {
    url: String,
    provider: Provider,
    connection: Connection
}

impl Database {
    pub fn new(url: String, provider: Provider) -> Database {
        Database {
            url,
            provider,
        }
    }

    pub fn connect(&self) {
        self.connection = self.provider.connect(self.url).unwrap_or_else(|err| {
            panic!("Error connecting to database: {}", err);
        });
    }

    pub fn disconnect(&self) {
        self.connection.disconnect(); // This approach will allow us to update the method in the future without changing the code that uses it.
    }
}

let db = Database::new("http://localhost:8080".to_string(), Provider::Postgres);
db.connect();
db.disconnect(); // Without clean code, we would have to change this line if we change the disconnect method in every place we use it.
```

### Glossary

- **Concretion**: A concrete object or instance of a class.
- **Polymorphism**: The ability to present the same interface for different data types.
