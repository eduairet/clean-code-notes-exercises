# Functions & Methods

- Both the function definition and the function call should be readable

  ```RUST
  // Function definition
  fn add_two_numbers(a: i32, b: i32) -> i32 {
      a + b
  }

  // Function call
  let sum = add_two_numbers(1, 2);
  ```

- The number of parameters should be minimized
- The parameters should be logically ordered
- More than 3 parameters should be avoided since it makes the function call less readable
- When we need more than 3 parameters, we can use an object to group the parameters

  ```Typescript
  class Rectangle {
        constructor(rectangle: { origin: Point, width: number, height: number }) {
            this,origin = rectangle.origin;
            this.width = rectangle.width;
            this.height = rectangle.height;
        }
  }

  const getRectangleArea = (rectangle: Rectangle): number => rectangle.width * rectangle.height;

  let rectangle = new Rectangle({ origin: { x: 0, y: 0 }, width: 10, height: 20 });
  let area = getRectangleArea(rectangle);
  ```

- When we have dynamic parameters, we can use each programming language's feature to handle them

  ```Python
  get_total_with_tip = lambda *args: sum(args) * 1.15
  total = get_total_with_tip(1, 2, 3, 4, 5)
  ```

- Avoid using output parameters

  ```RUST
  // Not ideal since user is modified in the function
  fn create_id(user: &mut User) {
     user.id = 1;
  }

  // Better: The function implies that the user is modified
  fn add_id(user: &User) -> User {
      User { id: 1, ..user }
  }

  // Great: The function implies that the user is modified
  let user = User { name: "John" };
  user.add_id();
  ```

- Functions should be small and do one thing, if a function is too long, it should be split into smaller functions, this is called the Single Responsibility Principle and it consists in add levels of abstraction to the code

  ```RUST
  fn validate_user(user: User) -> Result<User, String> {
      if user.name.is_empty() {
          return Err("Name is required".to_string());
      }
      if user.age < 18 {
          return Err("User must be at least 18 years old".to_string());
      }
      Ok(user)
  }

  fn get_user_by_email(email: String) -> User {
      // Get user from database
  }

  fn login_user(credentials: Credentials) -> Result<User, String> {
      let user = get_user_by_email(credentials.email);
      let user = validate_user(user)?;
      Ok(user)
  }
  ```

  - Knowing when to split a function is a matter of experience, but a good rule of thumb is to keep everything just one level of abstraction below the stated name of the function, in the example above, the `login_user` function is responsible for logging in a user, so it should not be responsible for validating the user, that's why the `validate_user` function was created

- Functions are a great way to follow the DRY (Don't Repeat Yourself) principle, if you find yourself repeating code, you should create a function to avoid updating the same code in multiple places
  - If you're unsure if you should create a function, you can ask yourself if by splitting the code into a new function, you're making the code more readable and easier to understand or if it's just adding complexity

## Side Effects

- Pure functions are functions that don't have side effects and always return the same output for the same input
- Impure functions are functions that have side effects and can return different outputs for the same input
  - For example a function that creates a random number is impure since it can return different outputs for the same input
- Functions should not have side effects, a function should only return a value and not modify any external state

  ```RUST
  // Not ideal since the function modifies the user
  fn add_id(user: &mut User) {
      user.id = 1;
  }

  // Better: The function implies that the user is modified
  fn add_id(user: &User) -> User {
      User { id: 1, ..user }
  }

  // Great: The function implies that the user is modified
  let user = User { name: "John" };
  user.add_id();
  ```

  - If you need a function that has side effects, you should make it clear in the function name

- Side effects make the code harder to reason about and test, since you need to know the state of the system before calling the function
- Most of the time we won't need to modify the state of the system through functions, we can use objects to handle the state and modify it through methods
- A good example of side effects is handling errors, if a function can return an error, it should return a Result type instead of throwing an exception

  ```RUST
  fn divide(a: i32, b: i32) -> Result<i32, String> {
      if b == 0 {
          return Err("Cannot divide by zero".to_string());
      }
      Ok(a / b)
  }

  let result = divide(10, 0);
  match result {
      Ok(value) => println!("Result: {}", value),
      Err(error) => println!("Error: {}", error),
  }
  ```

- Another way to keep clean functions is by unit testing them, if a function is hard to test, it's a sign that the function is doing too much and should be split into smaller functions
