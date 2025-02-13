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

- Functions should be small and do one thing, if a function is too long, it should be split into smaller functions

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
