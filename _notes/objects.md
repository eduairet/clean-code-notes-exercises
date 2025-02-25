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

- Polymorphism is a powerful tool to avoid using if-else statements.

```Rust
trait Cat {
    fn sound(&self) -> String;
}

struct Siamese;
struct Persian;

impl Cat for Siamese {
    fn sound(&self) -> String {
        String::from("Meow")
    }
}

impl Cat for Persian {
    fn sound(&self) -> String {
        String::from("Purr")
    }
}

fn make_sound(cat: &dyn Cat) {
    println!("{}", cat.sound());
}

let siamese = Siamese;
let persian = Persian;

make_sound(&siamese);
make_sound(&persian);
```

- Classes, as functions, should be small and should have a **single responsibility (Single Responsibility Principle)**.

```Rust
pub struct Order {
    items: Vec<Item>,
    total: f64,
}

// These methods have the only responsibility of managing the items and the total of the order.
impl Order {
    pub fn new() -> Order {
        Order {
            items: Vec::new(),
            total: 0.0,
        }
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
        self.total += item.price;
    }

    pub fn remove_item(&mut self, item: Item) {
        if let Some(index) = self.items.iter().position(|i| i == &item) {
            self.items.remove(index);
            self.total -= item.price;
        }
    }

    pub fn total(&self) -> f64 {
        self.total
    }
}

pub struct Item {
    name: String,
    price: f64,
}

impl Item {
    pub fn new(name: String, price: f64) -> Item {
        Item { name, price }
    }
}
```

- Another important concept when writing classes is **Cohesion**. Classes should have high cohesion, meaning that the methods and fields of the class should be related to each other.

  ```Rust
  pub struct User {
      username: String,
      email: String,
      password: String,
  }

  impl User {
      pub fn new(username: String, email: String, password: String) -> User {
          User {
              username,
              email,
              password,
          }
      }

      pub fn change_username(&mut self, new_username: String) {
          self.username = new_username;
      }

      pub fn change_email(&mut self, new_email: String) {
          self.email = new_email;
      }

      pub fn change_password(&mut self, new_password: String) {
          self.password = new_password;
      }
  }
  ```

  - Maximum cohesion is when all the methods and fields are related to each other.
  - No cohesion is when the methods and fields are not related to the class, for example, utility classes that are used in many places.
  - The goal is to have high cohesion classes.

- The **Law of Demeter** states that a method of an object should only call methods of:

  - Principle of Least Knowledge: Don't depend of internal objects that you don't know.[^1]
  - Code in a method should only access the following objects:

    - The object itself.
    - Objects that are stored in fields of the object.
    - Objects that are passed as arguments to the method.
    - Objects that are created inside the method.

    ```Rust
    // self.user.birthdate.date; // Violates the Law of Demeter because it accesses the birthdate object of the user object.

    struct Customer {
        pub last_purchase: Purchase,
    }

    struct DeliveryJob {
        customer: Customer,
        warehouse: Warehouse,
    }

    impl DeliveryJob {
        fn new(customer: Customer, warehouse: Warehouse) -> DeliveryJob {
            DeliveryJob {
                customer,
                warehouse,
            }
        }

        fn deliver_last_purchase(&self) {
            // let date = self.customer.lastPurchase.date; // Violates the Law of Demeter because it accesses the date object of the lastPurchase object.
            // let date = self.customer.getLastPurchaseDate();  // This is better because it hides the implementation details of the customer object but it can become dirty if the project grows.
            // self.warehouse.deliverPurchasesByDate(self.customer, date); // This is a much better approach because it hides the implementation details of the customer object and the warehouse object.
            self.warehouse.deliver_purchase(self.customer.last_purchase);
        }
    }
    ```

    - In a few words, **"Tell, don't ask"**. Don't ask for the internal details of an object, tell the object to do something.

## SOLID Principles

- **Single Responsibility Principle**: A class should have
  - Only one reason to change.
  - Only one responsibility.
- **Open/Closed Principle**: A class should be open for extension but closed for modification.
  - This ought us to use polymorphism and inheritance to create new classes instead of modifying existing huge classes, which will also help to write DRY code.
- **Liskov Substitution Principle**: Objects of a superclass should be replaceable with objects of a subclass without affecting the correctness of the program.
  - It forces you to model your data and classes correctly.
- **Interface Segregation Principle**: Many client-specific interfaces are better than one general-purpose interface.
  - It forces you to create many small interfaces instead of a big one.
  - It helps to avoid the **God Object** anti-pattern.
- **Dependency Inversion Principle**: You should depend on abstractions, not on concretions.
  - It forces you to use interfaces instead of concrete classes.
  - It helps to write code that is easier to test and maintain.

### Glossary

- **Concretion**: A concrete object or instance of a class.
- **Polymorphism**: The ability to present the same interface for different data types.

### References

[^1]: Data structures are an exception to this rule because they are just data containers and don't have any logic.
