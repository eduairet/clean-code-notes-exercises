/// Bird trait is a base trait for all birds.
pub trait Bird {
    fn new(name: &str) -> Self
    where
        Self: Sized;

    fn name(&self) -> &str;
}

/// FlyingBird trait is a trait for birds that can fly.
pub trait FlyingBird: Bird {
    fn fly(&self);
}

/// Eagle struct
///
/// # Example
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_06::lsp::*;
///
/// let eagle = Eagle::new("Royal Eagle");
/// assert_eq!(eagle.name(), "Royal Eagle");
/// assert_eq!(eagle.fly(), ());
/// assert_eq!(eagle.dive(), ());
/// ```
pub struct Eagle {
    name: String,
}

impl Bird for Eagle {
    fn new(name: &str) -> Self {
        Eagle {
            name: name.to_string(),
        }
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl FlyingBird for Eagle {
    fn fly(&self) {
        println!("Flying...");
    }
}

impl Eagle {
    pub fn dive(&self) {
        println!("Diving...");
    }
}

/// Penguin struct
///
/// # Example
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_06::lsp::*;
///
/// let penguin = Penguin::new("Emperor Penguin");
/// assert_eq!(penguin.name(), "Emperor Penguin");
/// ```
pub struct Penguin {
    pub name: String,
}

impl Bird for Penguin {
    fn new(name: &str) -> Self {
        Penguin {
            name: name.to_string(),
        }
    }

    fn name(&self) -> &str {
        &self.name
    }
}
