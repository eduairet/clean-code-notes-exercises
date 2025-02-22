use crate::exercises::exercise_05::enums::delivery_action::DeliveryAction;

/// An enum representing the type of delivery.
///
/// # Example
///
/// ```
/// use clean_code_notes_exercises::exercises::delivery_type::DeliveryType;
///
/// let delivery_type = DeliveryType::Express;
///
/// assert_eq!(delivery_type, DeliveryType::Express);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum DeliveryType {
    Express,
    Insured,
    Standard,
}

impl DeliveryType {
    /// Returns a string representation of the delivery type.
    ///
    /// # Example
    ///
    /// ```
    /// use clean_code_notes_exercises::exercises::{delivery_type::DeliveryType, delivery_action::DeliveryAction};
    ///
    /// let delivery_type = DeliveryType::Express;
    ///
    /// let action = DeliveryAction::Issue;
    /// let product = "A product".to_string();
    ///
    /// let result = delivery_type.get_action_message(action, product);
    /// assert_eq!(result, "Issuing express delivery for A product");
    /// ```
    pub fn get_action_message(&self, action: DeliveryAction, product: String) -> String {
        match self {
            DeliveryType::Express => match action {
                DeliveryAction::Issue => {
                    format!("Issuing express delivery for {}", product)
                }
                DeliveryAction::Track => {
                    format!("Tracking express delivery for {}", product)
                }
            },
            DeliveryType::Insured => match action {
                DeliveryAction::Issue => {
                    format!("Issuing insured delivery for {}", product)
                }
                DeliveryAction::Track => {
                    format!("Tracking insured delivery for {}", product)
                }
            },
            DeliveryType::Standard => match action {
                DeliveryAction::Issue => {
                    format!("Issuing standard delivery for {}", product)
                }
                DeliveryAction::Track => {
                    format!("Tracking standard delivery for {}", product)
                }
            },
        }
    }
}
