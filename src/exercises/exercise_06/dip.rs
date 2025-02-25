/// Use this trait on every database struct
pub trait Database {
    fn store_data(&self, data: &str) {
        println!("Storing data: {}", data);
    }
    // With dependency inversion principle the base Database won't have the connect method
    // This way the base Database trait can be used for both local and remote databases
}

/// Remote database trait, use this trait on every remote database struct
pub trait RemoteDatabase: Database {
    fn new(uri: &str) -> Self
    where
        Self: Sized;

    fn uri(&self) -> &str;

    fn connect(&self) {
        println!("Connecting to database: {}", self.uri());
    }
}

/// SQL database struct
///
/// # Example
/// ```
/// use clean_code_notes_exercises::exercises::exercise_06::dip::*;
///
/// let sql_database = SQLDatabase::new("http://localhost:8080");
/// assert_eq!(sql_database.uri, "http://localhost:8080");
/// assert_eq!(sql_database.connect(), ());
/// assert_eq!(sql_database.store_data("Some data"), ());
/// ```
pub struct SQLDatabase {
    pub uri: String,
}

impl Database for SQLDatabase {}

impl RemoteDatabase for SQLDatabase {
    fn new(uri: &str) -> Self {
        SQLDatabase {
            uri: uri.to_string(),
        }
    }

    fn uri(&self) -> &str {
        &self.uri
    }

    fn connect(&self) {
        std::println!("Connecting to database: {}", self.uri());
    }
}

/// In-memory database struct
///
/// Implements the `Database` trait for an in-memory database.
///
/// # Example
/// ```
/// use clean_code_notes_exercises::exercises::exercise_06::dip::*;
///
/// let in_memory_database = InMemoryDatabase;
/// assert_eq!(in_memory_database.store_data("Some data"), ());
/// ```
pub struct InMemoryDatabase;

impl Database for InMemoryDatabase {}

/// App struct
///
/// # Example
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_06::dip::*;
///
/// let sql_database = SQLDatabase::new("http://localhost:8080");
/// let app = App::new(sql_database);
/// assert_eq!(app.save_settings(), ());
/// assert_eq!(app.database.uri, "http://localhost:8080");
/// assert_eq!(app.database.connect(), ());
/// assert_eq!(app.database.store_data("Some data"), ());
/// ```
pub struct App<D: Database> {
    pub database: D,
}

impl<D: Database> App<D> {
    pub fn new(database: D) -> Self {
        App { database }
    }

    pub fn save_settings(&self) {
        self.database.store_data("Some data");
    }
}
