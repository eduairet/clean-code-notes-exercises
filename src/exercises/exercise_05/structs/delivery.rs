use crate::exercises::{delivery_type::DeliveryType, purchase::Purchase};

/// A delivery struct.
///
/// # Example
///
/// ```
/// use clean_code_notes_exercises::exercises::{delivery::Delivery, delivery_type::DeliveryType, purchase::Purchase};
///
/// let purchase = Purchase {
///    product: "A product".to_string(),
///    delivery_type: DeliveryType::Express,
/// };
///
/// let delivery = Delivery::new(purchase);
///
/// delivery.deliver_product();
/// delivery.track_product();
/// ```
#[derive(Debug)]
pub struct Delivery {
    purchase: Purchase,
}

impl Delivery {
    pub fn new(purchase: Purchase) -> Self {
        Self { purchase }
    }

    pub fn deliver_product(&self) {
        match self.purchase.delivery_type {
            DeliveryType::Express => {
                println!("Issuing express delivery for {}", self.purchase.product)
            }
            DeliveryType::Insured => {
                println!("Issuing insured delivery for {}", self.purchase.product)
            }
            DeliveryType::Standard => {
                println!("Issuing standard delivery for {}", self.purchase.product)
            }
        }
    }

    pub fn track_product(&self) {
        match self.purchase.delivery_type {
            DeliveryType::Express => {
                println!("Tracking express delivery for {}", self.purchase.product)
            }
            DeliveryType::Insured => {
                println!("Tracking insured delivery for {}", self.purchase.product)
            }
            DeliveryType::Standard => {
                println!("Tracking standard delivery for {}", self.purchase.product)
            }
        }
    }
}
