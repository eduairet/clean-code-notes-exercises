# Naming - Assigning Names to Variables, Functions, Classes & More

- Names should be meaningful and descriptive.
- Well-named code allows readers to understand the code without having to read the implementation details.

## General Naming Guidelines

- Variable names:
  - Use nouns or short phrases with adjectives related to the data they hold.
    ```RUST
    let name: String = "John Doe".to_string();
    let age: u8 = 30;
    let is_adult: bool = true;
    ```
- Function names:

  - Use verbs or short phrases with adverbs that describe the action the function performs.

    ```RUST
    fn calculate_area(length: f64, width: f64) -> f64 {
        length * width
    }

    fn user_is_adult(age: u8) -> bool {
        age >= 18
    }
    ```

- Class names:

  - Use nouns or noun phrases that describe the class.

    ```RUST
    struct Person {
        name: String,
        age: u8,
    }

    struct RequestBody {
        id: u32,
        name: String,
    }
    ```

- Name casing varies by language, check the best practices of the language you're using here are some common conventions:

  - `snake_case`:
    ```RUST
    let first_name: String = "John".to_string();
    ```
  - `camelCase`:
    ```TypeScript
    function calculateArea(length: number, width: number): number {
        return length * width;
    }
    ```
  - `PascalCase`:
    ```C#
    public enum UserType
    {
        Admin,
        User,
    }
    ```
  - `kebab-case`:
    ```HTML
    <my-component></my-component>
    ```
  - `UPPER_CASE`:
    ```Python
    # Constants
    MAX_RETRIES = 3
    ```

- Avoid using single-letter names unless they are used as counters or iterators.

  ```RUST
  // Bad
  let x: u8 = 10;

  // Good
  let age: u8 = 10;
  ```

- A good way to decide how to name something is to think about how you would describe

  - If the value is an object, number, or string, describe what it is.
    ```RUST
    // This is a bad name
    let p: Person = Person::new("John Doe", 30, false);
    // This can work but it's better to be more specific.
    let person: Person = Person::new("John Doe", 30, false);
    // Always try to include the specificity in the name.
    let subscriber: User = User::new("Bob", 25);
    ```
  - If the value is a boolean, respond a yes/no question.
    ```RUST
    let age: u8 = 30;
    // This is a bad name
    let a: bool = age >= 18;
    // This can work but it's better to be more specific.
    let adult: bool = age >= 18;;
    // This is better because it answers the question
    let is_adult: bool = age >= 18;;
    ```
  - If the value is a function, describe what it does.

    ```RUST
    // This is a bad name
    fn process(name: String, age: u8) -> Person {
        Person::new(name, age, false)
    }
    // This can work but it's better to be more specific.
    fn save(name: String, age: u8) -> Person {
        Person::new(name, age, false)
    }

    // Always try to include the specificity in the name.
    fn save_user(name: String, age: u8) -> Person {
        Person::new(name, age, false)
    }
    ```

  - If the function computes a boolean, start the name with `is`, `has`, or `can`.
    ```RUST
    // This is a bad name
    fn f(a: u8) -> bool {
        a >= 18
    }
    // This can work but it's better to be more specific.
    fn is_adult(a: u8) -> bool {
        a >= 18
    }
    // Always try to include the specificity in the name.
    fn is_user_adult(a: u8) -> bool {
        a >= 18
    }
    ```

- If the element is an object, we should describe what it is without redundancy.

  ```RUST
  // Bad name - Redundant and not descriptive
  struct DataStorage {
    connection_string: String;
  }

  // This can work but it's better to be more specific.
  struct Data {
    connection_string: String;
  }

  // Always try to include the specificity in the name.
  struct Database {
    connection_string: String;
  }
  ```

- If the object contains static methods, it's a good practice to include a suffix like `Utils` to indicate that it's a utility class, even if it can look redundant.

  ```C#
  public class MathUtils
  {
      public static int Add(int a, int b)
      {
          return a + b;
      }

      public static int Subtract(int a, int b)
      {
          return a - b;
      }
  }
  ```

- Also if the method turns to be a property but it's a method, like a getter, it's a good practice to just name it as a property.

  ```TypeScript
  class Database {
    private _client: any;

    get connectedClient(): any {
      if (!this._client) throw new Error('Database is not connected');
      return this._client;
    }

    connect(connectionString: string): void {
      this._client = connect(connectionString);
    }
  }
  ```
