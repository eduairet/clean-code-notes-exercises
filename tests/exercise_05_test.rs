use clean_code_notes_exercises::exercises::exercise_05::Delivery::{
    Delivery, DeliveryExpress, DeliveryInsured, DeliveryStandard, DeliveryTrait,
};

#[cfg(test)]
mod exercise_05_test {
    use super::*;

    #[test]
    fn test_delivery_express() {
        let delivery = DeliveryExpress {
            base: Delivery::new("A product".to_string()),
        };

        let issue = delivery.deliver_product();
        assert_eq!(issue, "Issuing express delivery for A product");

        let track = delivery.track_product();
        assert_eq!(track, "Tracking express delivery for A product");
    }

    #[test]
    fn test_delivery_insured() {
        let delivery = DeliveryInsured {
            base: Delivery::new("A product".to_string()),
        };

        let issue = delivery.deliver_product();
        assert_eq!(issue, "Issuing insured delivery for A product");

        let track = delivery.track_product();
        assert_eq!(track, "Tracking insured delivery for A product");
    }

    #[test]
    fn test_delivery_standard() {
        let delivery = DeliveryStandard {
            base: Delivery::new("A product".to_string()),
        };

        let issue = delivery.deliver_product();
        assert_eq!(issue, "Issuing standard delivery for A product");

        let track = delivery.track_product();
        assert_eq!(track, "Tracking standard delivery for A product");
    }
}
