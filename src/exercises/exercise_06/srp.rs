/// User struct with login, signup and assign_role methods
///
/// # Example
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_06::srp::*;
///
/// let mut member = Member::new("test@test.com", "password");
/// assert_eq!(member.signup("test@test.com", "password"), "Member test@test.com signed up");
/// assert_eq!(member.assign_role("admin"), "Member test@test.com assigned role admin");
/// assert_eq!(member.login("test@test.com", "password"), true);
/// ```
pub struct Member {
    email: String,
    password: String,
    role: String,
}

impl Member {
    pub fn new(email: &str, password: &str) -> Self {
        Member {
            email: email.to_string(),
            password: password.to_string(),
            role: "user".to_string(),
        }
    }

    pub fn login(&self, email: &str, password: &str) -> bool {
        if self.email.is_empty() || self.password.is_empty() {
            return false;
        }
        email == self.email && password == self.password
    }

    pub fn signup(&mut self, email: &str, password: &str) -> String {
        self.email = email.to_string();
        self.password = password.to_string();
        format!("Member {} signed up", email)
    }

    pub fn assign_role(&mut self, role: &str) -> String {
        self.role = role.to_string();
        format!("Member {} assigned role {}", self.email, role)
    }
}
