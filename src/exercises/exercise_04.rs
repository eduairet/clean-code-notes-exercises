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
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, PartialEq, Clone)]
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
#[derive(Debug, PartialEq, Clone)]
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
/// use clean_code_notes_exercises::exercises::exercise_04::*;
/// use clean_code_notes_exercises::transaction_closed_error;
///
/// assert_eq!(format!("{}", TransactionError::Empty), "There are no transactions to process");
/// assert_eq!(format!("{}", TransactionError::Closed(None)), "Your transaction is already closed");
/// assert_eq!(format!("{}", transaction_closed_error!()), "Your transaction is already closed");
///
/// let payment_transaction = Transaction::new(
///   String::from("t1"),
///   TransactionType::Payment,
///   TransactionStatus::Closed,
///   PaymentMethod::CreditCard,
///   23.99,
/// );
///
/// let payment_expected_error = TransactionError::Closed(Some(payment_transaction.clone()));
///
/// assert_eq!(format!("{}", payment_expected_error), "Your payment is already closed | Transaction: id: t1, type: payment, status: closed, method: credit card, amount: 23.99");
///
/// let refund_transaction = Transaction::new(
///   String::from("t2"),
///   TransactionType::Refund,
///   TransactionStatus::Closed,
///   PaymentMethod::PayPal,
///   100.43,
/// );
///
/// let refund_expected_error = TransactionError::Closed(Some(refund_transaction.clone()));
///
/// assert_eq!(format!("{}", payment_expected_error), "Your payment is already closed | Transaction: id: t1, type: payment, status: closed, method: credit card, amount: 23.99");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionError {
    Empty,
    Closed(Option<Transaction>),
}

#[macro_export]
macro_rules! transaction_closed_error {
    () => {
        TransactionError::Closed(None)
    };
    ($transaction:expr) => {
        TransactionError::Closed(Some($transaction))
    };
}

impl std::fmt::Display for TransactionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TransactionError::Empty => write!(f, "There are no transactions to process"),
            TransactionError::Closed(transaction) => {
                write!(
                    f,
                    "{}",
                    match transaction {
                        Some(t) => format!("Your {} is already closed | {}", t.transaction_type, t),
                        None => "Your transaction is already closed".to_string(),
                    }
                )
            }
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
/// assert_eq!(closed_transaction.process(), Err(TransactionError::Closed(Some(closed_transaction.clone()))));
/// assert_eq!(format!("{}", transaction), "Transaction: id: t1, type: payment, status: open, method: credit card, amount: 23.99");
/// ```
#[derive(Debug, Clone, PartialEq)]
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

    fn get_processor(&self) -> String {
        format!(
            "Processing {} {} for amount: {:.2}",
            self.transaction_type, self.method, self.amount
        )
    }

    pub fn process(&self) -> Result<String, TransactionError> {
        match self.status {
            TransactionStatus::Closed => {
                return Err(TransactionError::Closed(Some(self.clone())));
            }
            TransactionStatus::Open => Ok(self.get_processor()),
        }
    }
}

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Transaction: id: {}, type: {}, status: {}, method: {}, amount: {}",
            self.id, self.transaction_type, self.status, self.method, self.amount
        )
    }
}

/// Validates a list of transactions.
///
/// # Example
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_04::*;
///
/// let transactions = vec![
///    Transaction {
///       id: String::from("t1"),
///       transaction_type: TransactionType::Payment,
///       status: TransactionStatus::Open,
///       method: PaymentMethod::CreditCard,
///       amount: 23.99,
///   },
/// ];
///
/// assert_eq!(validate_transactions(&transactions), Ok(()));
///
/// let empty_transactions: Vec<Transaction> = vec![];
/// assert_eq!(validate_transactions(&empty_transactions), Err(TransactionError::Empty));
/// ```
pub fn validate_transactions(transactions: &Vec<Transaction>) -> Result<(), TransactionError> {
    match transactions.is_empty() {
        true => Err(TransactionError::Empty),
        false => Ok(()),
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
///         status: TransactionStatus::Closed,
///         method: PaymentMethod::Plan,
///         amount: 15.99,
///     },
/// ];
///
/// let expected = vec![
///    "Processing payment credit card for amount: 23.99".to_string(),
///    "Your payment is already closed | Transaction: id: t2, type: payment, status: closed, method: plan, amount: 15.99".to_string(),
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
    validate_transactions(transactions)?;

    let results: Vec<String> = transactions
        .iter()
        .map(|transaction| transaction.process().unwrap_or_else(|e| e.to_string()))
        .collect();

    Ok(results)
}
