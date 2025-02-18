use clean_code_notes_exercises::exercises::exercise_02::*;

#[cfg(test)]
mod exercise_02_test {
    use super::*;

    const VALID_USERNAME: &str = "username";
    const VALID_EMAIL: &str = "test@test.com";
    const VALID_PASSWORD: &str = "password";
    const INVALID_USERNAME: &str = "user name";
    const INVALID_EMAIL: &str = "test@test";
    const INVALID_PASSWORD: &str = "pass";

    #[test]
    fn test_create_user_pass() {
        let result = create_user(
            VALID_USERNAME.to_string(),
            VALID_EMAIL.to_string(),
            VALID_PASSWORD.to_string(),
        );
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_create_user_fail_username() {
        let result = create_user(
            INVALID_USERNAME.to_string(),
            VALID_EMAIL.to_string(),
            VALID_PASSWORD.to_string(),
        );
        assert_eq!(result.is_err(), true);
        assert_eq!(result.unwrap_err(), CreateUserError::InvalidUsername);
    }

    #[test]
    fn test_create_user_fail_email() {
        let result = create_user(
            VALID_USERNAME.to_string(),
            INVALID_EMAIL.to_string(),
            VALID_PASSWORD.to_string(),
        );
        assert_eq!(result.is_err(), true);
        assert_eq!(result.unwrap_err(), CreateUserError::InvalidEmail);
    }

    #[test]
    fn test_create_user_fail_password() {
        let result = create_user(
            VALID_USERNAME.to_string(),
            VALID_EMAIL.to_string(),
            INVALID_PASSWORD.to_string(),
        );
        assert_eq!(result.is_err(), true);
        assert_eq!(result.unwrap_err(), CreateUserError::InvalidPassword);
    }

    #[test]
    fn test_create_user_fail_multiple() {
        let test_cases = vec![
            (
                INVALID_USERNAME,
                INVALID_EMAIL,
                VALID_PASSWORD,
                CreateUserError::InvalidUserData,
            ),
            (
                INVALID_USERNAME,
                VALID_EMAIL,
                INVALID_PASSWORD,
                CreateUserError::InvalidUserData,
            ),
            (
                VALID_USERNAME,
                INVALID_EMAIL,
                INVALID_PASSWORD,
                CreateUserError::InvalidUserData,
            ),
            (
                INVALID_USERNAME,
                INVALID_EMAIL,
                INVALID_PASSWORD,
                CreateUserError::InvalidUserData,
            ),
        ];

        for (username, email, password, expected_error) in test_cases {
            let result = create_user(
                username.to_string(),
                email.to_string(),
                password.to_string(),
            );
            assert_eq!(result.is_err(), true);
            assert_eq!(result.unwrap_err(), expected_error);
        }
    }
}
