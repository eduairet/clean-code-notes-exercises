/// TransactionType enum
///
/// # Examples
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_04::TransactionType;
///
/// assert_eq!(format!("{}", TransactionType::Payment), "payment");
/// assert_eq!(format!("{}", TransactionType::Refund), "refund");
/// ```
#[derive(Debug, PartialEq)]
pub enum TransactionType {
    Payment,
    Refund,
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TransactionType::Payment => write!(f, "payment"),
            TransactionType::Refund => write!(f, "refund"),
        }
    }
}

/// TransactionStatus enum
///
/// # Examples
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_04::TransactionStatus;
///
/// assert_eq!(format!("{}", TransactionStatus::Open), "open");
/// assert_eq!(format!("{}", TransactionStatus::Closed), "closed");
/// ```
#[derive(Debug, PartialEq)]
pub enum TransactionStatus {
    Open,
    Closed,
}

impl std::fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TransactionStatus::Open => write!(f, "open"),
            TransactionStatus::Closed => write!(f, "closed"),
        }
    }
}

/// PaymentMethod enum
///
/// # Examples
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_04::PaymentMethod;
///
/// assert_eq!(format!("{}", PaymentMethod::CreditCard), "credit card");
/// assert_eq!(format!("{}", PaymentMethod::PayPal), "PayPal");
/// assert_eq!(format!("{}", PaymentMethod::Plan), "plan");
/// ```
#[derive(Debug, PartialEq)]
pub enum PaymentMethod {
    CreditCard,
    PayPal,
    Plan,
}

impl std::fmt::Display for PaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PaymentMethod::CreditCard => write!(f, "credit card"),
            PaymentMethod::PayPal => write!(f, "PayPal"),
            PaymentMethod::Plan => write!(f, "plan"),
        }
    }
}

/// TransactionError enum
///
/// # Examples
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_04::TransactionError;
///
/// assert_eq!(format!("{}", TransactionError::Empty), "There are no transactions to process");
/// assert_eq!(format!("{}", TransactionError::PaymentClosed), "Your payment is already closed");
/// assert_eq!(format!("{}", TransactionError::RefundClosed), "Your refund is already closed");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionError {
    Empty,
    PaymentClosed,
    RefundClosed,
}

impl std::fmt::Display for TransactionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TransactionError::Empty => write!(f, "There are no transactions to process"),
            TransactionError::PaymentClosed => write!(f, "Your payment is already closed"),
            TransactionError::RefundClosed => write!(f, "Your refund is already closed"),
        }
    }
}

/// Transaction struct
///
/// # Examples
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_04::*;
///
/// let transaction = Transaction::new(
///    String::from("t1"),
///    TransactionType::Payment,
///    TransactionStatus::Open,
///    PaymentMethod::CreditCard,
///    23.99,
/// );
///
/// assert_eq!(transaction.id, "t1");
/// assert_eq!(transaction.transaction_type, TransactionType::Payment);
/// assert_eq!(transaction.status, TransactionStatus::Open);
/// assert_eq!(transaction.method, PaymentMethod::CreditCard);
/// assert_eq!(transaction.amount, 23.99);
///
/// assert_eq!(transaction.process(), Ok("Processing payment credit card for amount: 23.99".to_string()));
///
/// let closed_transaction = Transaction::new(
///   String::from("t2"),
///   TransactionType::Payment,
///   TransactionStatus::Closed,
///   PaymentMethod::PayPal,
///   100.43,
/// );
///
/// assert_eq!(closed_transaction.process(), Err(TransactionError::PaymentClosed));
/// ```
pub struct Transaction {
    pub id: String,
    pub transaction_type: TransactionType,
    pub status: TransactionStatus,
    pub method: PaymentMethod,
    pub amount: f64,
}

impl Transaction {
    pub fn new(
        id: String,
        transaction_type: TransactionType,
        status: TransactionStatus,
        method: PaymentMethod,
        amount: f64,
    ) -> Transaction {
        Transaction {
            id,
            transaction_type,
            status,
            method,
            amount,
        }
    }

    pub fn process(&self) -> Result<String, TransactionError> {
        if self.status == TransactionStatus::Closed {
            return match self.transaction_type {
                TransactionType::Payment => Err(TransactionError::PaymentClosed),
                TransactionType::Refund => Err(TransactionError::RefundClosed),
            };
        };

        Ok(format!(
            "Processing {} {} for amount: {}",
            self.transaction_type, self.method, self.amount
        ))
    }
}

/// Processes a list of transactions.
///
/// # Example
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_04::*;
///
/// let transactions = vec![
///     Transaction {
///         id: String::from("t1"),
///         transaction_type: TransactionType::Payment,
///         status: TransactionStatus::Open,
///         method: PaymentMethod::CreditCard,
///         amount: 23.99,
///     },
///     Transaction {
///         id: String::from("t2"),
///         transaction_type: TransactionType::Payment,
///         status: TransactionStatus::Open,
///         method: PaymentMethod::PayPal,
///         amount: 100.43,
///     },
///     Transaction {
///         id: String::from("t3"),
///         transaction_type: TransactionType::Refund,
///         status: TransactionStatus::Open,
///         method: PaymentMethod::CreditCard,
///         amount: 10.99,
///     },
///     Transaction {
///         id: String::from("t4"),
///         transaction_type: TransactionType::Payment,
///         status: TransactionStatus::Closed,
///         method: PaymentMethod::Plan,
///         amount: 15.99,
///     },
/// ];
///
/// let expected = vec![
///    "Processing payment credit card for amount: 23.99".to_string(),
///    "Processing payment PayPal for amount: 100.43".to_string(),
///    "Processing refund credit card for amount: 10.99".to_string(),
///    "Your payment is already closed".to_string(),
/// ];
///
/// assert_eq!(process_transactions(&transactions), Ok(expected));
///
/// let empty_transactions: Vec<Transaction> = vec![];
/// assert_eq!(process_transactions(&empty_transactions), Err(TransactionError::Empty));
/// ```
pub fn process_transactions(
    transactions: &Vec<Transaction>,
) -> Result<Vec<String>, TransactionError> {
    if transactions.is_empty() {
        return Err(TransactionError::Empty);
    }

    let results: Vec<String> = transactions
        .iter()
        .map(|transaction| transaction.process().unwrap_or_else(|e| e.to_string()))
        .collect();

    Ok(results)
}
