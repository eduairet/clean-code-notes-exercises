/// Delivery action enum
///
/// # Example
///
/// ```
/// use clean_code_notes_exercises::exercises::delivery_action::DeliveryAction;
///
/// let delivery_action = DeliveryAction::Issue;
///
/// assert_eq!(delivery_action, DeliveryAction::Issue);
/// ```
#[derive(Debug, PartialEq)]
pub enum DeliveryAction {
    Issue,
    Track,
}
