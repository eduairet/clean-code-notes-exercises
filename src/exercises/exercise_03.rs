use regex::Regex;

/// User struct
///
/// # Examples
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_03::User;
///
/// let user = User::new("username".to_string(), "test@test.com".to_string(), "password".to_string());
/// assert_eq!(user.username, "username");
/// assert_eq!(user.email, "test@test.com");
/// assert_eq!(user.password, "password");
/// user.save();
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

    pub fn save(&self) -> () {
        println!("User saved: {:?}", self);
    }
}

/// CreateUserError enum
///
/// # Examples
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_03::{CreateUserError, create_user};
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
/// use clean_code_notes_exercises::exercises::exercise_03::create_user;
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
    Validate::user_data(&username, &email, &password)?;
    let new_user = User::new(username, email, password);
    Ok(new_user.save())
}

/// This struct contains validation functions for different inputs
///
/// # Examples
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_03::Validate;
///
/// assert_eq!(Validate::password("password"), true);
/// assert_eq!(Validate::password("pass"), false);
/// assert_eq!(Validate::username("12345"), true);
/// assert_eq!(Validate::username("123 43"), false);
/// assert_eq!(Validate::email("test@test"), false);
/// assert_eq!(Validate::email("test@test.com"), true);
/// assert_eq!(Validate::user_data("username", "test@test.com", "password").is_ok(), true);
/// assert_eq!(Validate::user_data("username", "test@test", "password").is_err(), true);
/// ```
pub struct Validate;

impl Validate {
    pub fn username(input: &str) -> bool {
        Regex::new(r"^[A-Za-z\d_]{5,}$").unwrap().is_match(input)
    }

    pub fn email(input: &str) -> bool {
        Regex::new(r"^[A-Za-z\d_]+@[A-Za-z\d_]+\.[A-Za-z\d_]+$")
            .unwrap()
            .is_match(input)
    }

    pub fn password(input: &str) -> bool {
        Regex::new(r"^[A-Za-z\d_]{5,}$").unwrap().is_match(input)
    }

    pub fn user_data(username: &str, email: &str, password: &str) -> Result<(), CreateUserError> {
        let mut errors = vec![];

        if !Validate::username(username) {
            errors.push(CreateUserError::InvalidUsername);
        }
        if !Validate::email(email) {
            errors.push(CreateUserError::InvalidEmail);
        }
        if !Validate::password(password) {
            errors.push(CreateUserError::InvalidPassword);
        }

        match errors.len() {
            0 => Ok(()),
            1 => Err(errors[0].clone()),
            _ => Err(CreateUserError::InvalidUserData),
        }
    }
}
