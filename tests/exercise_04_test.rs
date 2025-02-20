use clean_code_notes_exercises::exercises::exercise_04::*;

#[cfg(test)]
mod exercise_04_test {
    use super::*;

    #[test]
    fn test_payment_transaction_process() {
        let open_payment_transaction = Transaction::new(
            String::from("t1"),
            TransactionType::Payment,
            TransactionStatus::Open,
            PaymentMethod::CreditCard,
            23.99,
        );

        assert_eq!(
            open_payment_transaction.process(),
            Ok("Processing payment credit card for amount: 23.99".to_string())
        );

        let closed_payment_transaction = Transaction::new(
            String::from("t2"),
            TransactionType::Payment,
            TransactionStatus::Closed,
            PaymentMethod::PayPal,
            100.43,
        );

        assert_eq!(
            closed_payment_transaction.process(),
            Err(TransactionError::PaymentClosed)
        );
    }

    #[test]
    fn test_refund_transaction_process() {
        let open_refund_transaction = Transaction::new(
            String::from("t1"),
            TransactionType::Refund,
            TransactionStatus::Open,
            PaymentMethod::CreditCard,
            23.99,
        );

        assert_eq!(
            open_refund_transaction.process(),
            Ok("Processing refund credit card for amount: 23.99".to_string())
        );

        let closed_refund_transaction = Transaction::new(
            String::from("t2"),
            TransactionType::Refund,
            TransactionStatus::Closed,
            PaymentMethod::PayPal,
            100.43,
        );

        assert_eq!(
            closed_refund_transaction.process(),
            Err(TransactionError::RefundClosed)
        );
    }

    #[test]
    fn test_process_transactions() {
        let empty_transactions: Vec<Transaction> = vec![];
        assert_eq!(
            process_transactions(&empty_transactions),
            Err(TransactionError::Empty)
        );

        let all_transaction_types = vec![
            Transaction::new(
                String::from("t1"),
                TransactionType::Payment,
                TransactionStatus::Open,
                PaymentMethod::CreditCard,
                23.99,
            ),
            Transaction::new(
                String::from("t2"),
                TransactionType::Payment,
                TransactionStatus::Closed,
                PaymentMethod::CreditCard,
                100.43,
            ),
            Transaction::new(
                String::from("t3"),
                TransactionType::Refund,
                TransactionStatus::Open,
                PaymentMethod::CreditCard,
                23.99,
            ),
            Transaction::new(
                String::from("t4"),
                TransactionType::Refund,
                TransactionStatus::Closed,
                PaymentMethod::CreditCard,
                100.43,
            ),
            Transaction::new(
                String::from("t5"),
                TransactionType::Payment,
                TransactionStatus::Open,
                PaymentMethod::PayPal,
                23.99,
            ),
            Transaction::new(
                String::from("t6"),
                TransactionType::Payment,
                TransactionStatus::Closed,
                PaymentMethod::PayPal,
                100.43,
            ),
            Transaction::new(
                String::from("t7"),
                TransactionType::Refund,
                TransactionStatus::Open,
                PaymentMethod::PayPal,
                23.99,
            ),
            Transaction::new(
                String::from("t8"),
                TransactionType::Refund,
                TransactionStatus::Closed,
                PaymentMethod::PayPal,
                100.43,
            ),
            Transaction::new(
                String::from("t9"),
                TransactionType::Payment,
                TransactionStatus::Open,
                PaymentMethod::Plan,
                23.99,
            ),
            Transaction::new(
                String::from("t10"),
                TransactionType::Payment,
                TransactionStatus::Closed,
                PaymentMethod::Plan,
                100.43,
            ),
            Transaction::new(
                String::from("t11"),
                TransactionType::Refund,
                TransactionStatus::Open,
                PaymentMethod::Plan,
                23.99,
            ),
            Transaction::new(
                String::from("t12"),
                TransactionType::Refund,
                TransactionStatus::Closed,
                PaymentMethod::Plan,
                100.43,
            ),
        ];

        let expected = vec![
            "Processing payment credit card for amount: 23.99".to_string(),
            "Your payment is already closed".to_string(),
            "Processing refund credit card for amount: 23.99".to_string(),
            "Your refund is already closed".to_string(),
            "Processing payment PayPal for amount: 23.99".to_string(),
            "Your payment is already closed".to_string(),
            "Processing refund PayPal for amount: 23.99".to_string(),
            "Your refund is already closed".to_string(),
            "Processing payment plan for amount: 23.99".to_string(),
            "Your payment is already closed".to_string(),
            "Processing refund plan for amount: 23.99".to_string(),
            "Your refund is already closed".to_string(),
        ];

        assert_eq!(process_transactions(&all_transaction_types), Ok(expected));
    }
}
