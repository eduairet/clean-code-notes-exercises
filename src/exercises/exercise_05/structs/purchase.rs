use crate::exercises::delivery_type::DeliveryType;

/// A purchase struct.
///
/// # Example
///
/// ```
/// use clean_code_notes_exercises::exercises::{delivery_type::DeliveryType, purchase::Purchase};
///
/// let purchase = Purchase {
///    product: "A product".to_string(),
///    delivery_type: DeliveryType::Express,
/// };
///
/// assert_eq!(purchase.product, "A product");
/// assert_eq!(purchase.delivery_type, DeliveryType::Express);
/// ```
#[derive(Debug)]
pub struct Purchase {
    pub product: String,
    pub delivery_type: DeliveryType,
}
