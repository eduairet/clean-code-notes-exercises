use regex::Regex;

/// User struct
///
/// # Examples
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_02::User;
///
/// let user = User::new("username".to_string(), "test@test.com".to_string(), "password".to_string());
/// assert_eq!(user.username, "username");
/// assert_eq!(user.email, "test@test.com");
/// assert_eq!(user.password, "password");
/// ```
#[derive(Debug)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(username: String, email: String, password: String) -> User {
        User {
            username,
            email,
            password,
        }
    }
}

/// CreateUserError enum
///
/// # Examples
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_02::{CreateUserError, create_user};
///
/// let username = "username".to_string();
/// let email = "test.com".to_string();
/// let password = "password".to_string();
///
/// let result = create_user(username, email, password);
/// assert_eq!(result.unwrap_err(), CreateUserError::InvalidEmail);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum CreateUserError {
    InvalidUsername,
    InvalidEmail,
    InvalidPassword,
    InvalidUserData,
}

impl std::fmt::Display for CreateUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CreateUserError::InvalidUsername => write!(f, "Invalid username"),
            CreateUserError::InvalidEmail => write!(f, "Invalid email"),
            CreateUserError::InvalidPassword => write!(f, "Invalid password"),
            CreateUserError::InvalidUserData => write!(f, "Invalid user data"),
        }
    }
}

impl std::error::Error for CreateUserError {}

/// Create user function
///
/// # Examples
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_02::create_user;
///
/// let username = "username".to_string();
/// let email = "test@test.com".to_string();
/// let password = "password".to_string();
///
/// let result = create_user(username, email, password);
///
/// assert_eq!(result.is_ok(), true);
/// ```
pub fn create_user(
    username: String,
    email: String,
    password: String,
) -> Result<(), CreateUserError> {
    validate_user_data(&username, &email, &password)?;
    Ok(save_user(User::new(username, email, password)))
}

/// Is username valid function
///
/// # Examples
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_02::{is_username_valid};
///
/// let username = "username";
/// let result = is_username_valid(username);
/// assert_eq!(result, true);
/// ```
pub fn is_username_valid(username: &str) -> bool {
    let username_pattern = Regex::new(r"^[A-Za-z\d_]+$").unwrap();
    username_pattern.is_match(username)
}

/// Is email valid function
///
/// # Examples
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_02::{is_email_valid};
///
/// let email = "test@test.com";
/// let result = is_email_valid(email);
/// assert_eq!(result, true);
/// ```
pub fn is_email_valid(email: &str) -> bool {
    let email_pattern = Regex::new(r"^[A-Za-z\d_]+@[A-Za-z\d_]+\.[A-Za-z\d_]+$").unwrap();
    email_pattern.is_match(email)
}

/// Is password valid function
///
/// # Examples
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_02::{is_password_valid};
///
/// let password = "password";
/// let result = is_password_valid(password);
/// assert_eq!(result, true);
/// ```
pub fn is_password_valid(password: &str) -> bool {
    let password_pattern = Regex::new(r"^[A-Za-z\d_]{5,}$").unwrap();
    password_pattern.is_match(password)
}

/// Is user data valid function
///
/// # Examples
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_02::{validate_user_data, CreateUserError};
///
/// let username = "username";
/// let email = "test@test.com";
/// let password = "password";
///
/// let result = validate_user_data(username, email, password);
/// assert_eq!(result.is_ok(), true);
/// ```
pub fn validate_user_data(
    username: &str,
    email: &str,
    password: &str,
) -> Result<(), CreateUserError> {
    let mut errors = vec![];

    if !is_username_valid(username) {
        errors.push(CreateUserError::InvalidUsername);
    }
    if !is_email_valid(email) {
        errors.push(CreateUserError::InvalidEmail);
    }
    if !is_password_valid(password) {
        errors.push(CreateUserError::InvalidPassword);
    }

    match errors.len() {
        0 => Ok(()),
        1 => Err(errors[0].clone()),
        _ => Err(CreateUserError::InvalidUserData),
    }
}

/// Save user function
///
/// # Examples
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_02::{save_user, User};
///
/// let user = User::new("username".to_string(), "test@test.com".to_string(), "password".to_string());
/// save_user(user);
/// ```
pub fn save_user(user: User) -> () {
    println!("User saved: {:?}", user);
}
