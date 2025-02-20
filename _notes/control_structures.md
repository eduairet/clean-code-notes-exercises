# Control Structures & Errors

- Avoid deep nesting

  - Guard clauses are early returns that help to avoid deep nesting.

  ```rust
  fn guard_clauses(user: User) {
      if !user.is_active() {
          return;
      }

      // Do something
  }
  ```

- Use Factory Functions & Polymorphism

  ```rust
  fn create_user(name: &str, age: u8) -> Box<dyn User> {
      if age < 18 {
          Box::new(Child::new(name, age))
      } else {
          Box::new(Adult::new(name, age))
      }
  }
  ```

  - To make it clear, a factory function is a function that returns an instance of a class or struct.
  - Polymorphism is the ability to present the same interface for different data types.

- Prefer Positive Conditions

  ```rust
  is_empty() // Good
  has_no_items() // Bad
  ```

- Use Errors where you have errors

  ```rust
  fn read_file(file_path: &str) -> Result<String, String> {
      match fs::read_to_string(file_path) {
          Ok(content) => Ok(content),
          Err(error) => Err(format!("Error reading file: {}", error)),
      }
  }
  ```

  - Error as functions follow the rule of doing one thing.

- Use default parameter values when a function will mostly be called with the same arguments

  ```TypeScript
  fn create_user(name: &str, age: u8, is_active: bool = true) -> User {
      User {
          name: name.to_string(),
          age,
          is_active,
      }
  }
  ```

- Avoid using magic numbers and strings

  ```rust
  const MAX_RETRIES: u8 = 3; // You can define a constant instead of using a magic number
  const FILE_PATH: &str = "file.txt"; // Or a magic string
  ```
