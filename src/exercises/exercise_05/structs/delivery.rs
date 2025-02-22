use crate::exercises::{delivery_type::DeliveryType, DeliveryAction};

pub trait DeliveryTrait {
    fn deliver_product(&self) -> String;
    fn track_product(&self) -> String;
}

/// A delivery struct.
///
/// # Example
///
/// ```
/// use clean_code_notes_exercises::exercises::delivery::Delivery;
///
/// let delivery = Delivery::new("A product".to_string());
/// assert_eq!(delivery.product, "A product");
/// ```
#[derive(Debug)]
pub struct Delivery {
    pub product: String,
}

impl Delivery {
    pub fn new(product: String) -> Self {
        Self { product }
    }
}

/// A delivery struct for express delivery.
///
/// # Example
/// ```
/// use clean_code_notes_exercises::exercises::delivery::{Delivery, DeliveryExpress, DeliveryTrait};
/// use clean_code_notes_exercises::exercises::delivery_type::DeliveryType;
/// use clean_code_notes_exercises::exercises::DeliveryAction;
///
/// let delivery = Delivery::new("A product".to_string());
/// let delivery_express = DeliveryExpress { base: delivery };
///
/// let result = delivery_express.deliver_product();
/// assert_eq!(result, "Issuing express delivery for A product");
/// ```
pub struct DeliveryExpress {
    pub base: Delivery,
}

impl DeliveryTrait for DeliveryExpress {
    fn deliver_product(&self) -> String {
        DeliveryType::Express
            .get_action_message(DeliveryAction::Issue, self.base.product.to_string())
    }

    fn track_product(&self) -> String {
        DeliveryType::Express
            .get_action_message(DeliveryAction::Track, self.base.product.to_string())
    }
}

/// A delivery struct for insured delivery.
///
/// # Example
///
/// ```
/// use clean_code_notes_exercises::exercises::delivery::{Delivery, DeliveryInsured, DeliveryTrait};
/// use clean_code_notes_exercises::exercises::delivery_type::DeliveryType;
/// use clean_code_notes_exercises::exercises::DeliveryAction;
///
/// let delivery = Delivery::new("A product".to_string());
/// let delivery_insured = DeliveryInsured { base: delivery };
///
/// let result = delivery_insured.deliver_product();
/// assert_eq!(result, "Issuing insured delivery for A product");
/// ```
pub struct DeliveryInsured {
    pub base: Delivery,
}

impl DeliveryTrait for DeliveryInsured {
    fn deliver_product(&self) -> String {
        DeliveryType::Insured
            .get_action_message(DeliveryAction::Issue, self.base.product.to_string())
    }

    fn track_product(&self) -> String {
        DeliveryType::Insured
            .get_action_message(DeliveryAction::Track, self.base.product.to_string())
    }
}

/// A delivery struct for standard delivery.
///
/// # Example
///
/// ```
/// use clean_code_notes_exercises::exercises::delivery::{Delivery, DeliveryStandard, DeliveryTrait};
/// use clean_code_notes_exercises::exercises::delivery_type::DeliveryType;
/// use clean_code_notes_exercises::exercises::DeliveryAction;
///
/// let delivery = Delivery::new("A product".to_string());
/// let delivery_standard = DeliveryStandard { base: delivery };
///
/// let result = delivery_standard.deliver_product();
/// assert_eq!(result, "Issuing standard delivery for A product");
/// ```
pub struct DeliveryStandard {
    pub base: Delivery,
}

impl DeliveryTrait for DeliveryStandard {
    fn deliver_product(&self) -> String {
        DeliveryType::Standard
            .get_action_message(DeliveryAction::Issue, self.base.product.to_string())
    }

    fn track_product(&self) -> String {
        DeliveryType::Standard
            .get_action_message(DeliveryAction::Track, self.base.product.to_string())
    }
}
