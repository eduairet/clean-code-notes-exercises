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
#[derive(Debug, PartialEq)]
pub enum DeliveryType {
    Express,
    Insured,
    Standard,
}
