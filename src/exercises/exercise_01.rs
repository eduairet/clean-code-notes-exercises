/// Point struct
///
/// # Examples
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_01::Point;
///
/// let point = Point::new(1.0, 2.0);
///
/// assert_eq!(point.x, 1.0);
/// assert_eq!(point.y, 2.0);
/// ```
#[derive(Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

/// Bounds struct
///
/// # Examples
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_01::{Bounds, Point};
///
/// let top_left = Point::new(0.0, 0.0);
/// let top_right = Point::new(1.0, 0.0);
/// let bottom_right = Point::new(1.0, -1.0);
/// let bottom_left = Point::new(0.0, -1.0);
/// let bounds = Bounds::new(top_left, top_right, bottom_right, bottom_left);
///
/// assert_eq!(bounds.top_left.x, 0.0);
/// assert_eq!(bounds.top_left.y, 0.0);
/// assert_eq!(bounds.top_right.x, 1.0);
/// assert_eq!(bounds.top_right.y, 0.0);
/// assert_eq!(bounds.bottom_right.x, 1.0);
/// assert_eq!(bounds.bottom_right.y, -1.0);
/// assert_eq!(bounds.bottom_left.x, 0.0);
/// assert_eq!(bounds.bottom_left.y, -1.0);
/// ```
pub struct Bounds {
    pub top_left: Point,
    pub top_right: Point,
    pub bottom_right: Point,
    pub bottom_left: Point,
}

impl Bounds {
    pub fn new(
        top_left: Point,
        top_right: Point,
        bottom_right: Point,
        bottom_left: Point,
    ) -> Bounds {
        Bounds {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        }
    }
}

/// Rectangle struct
///
/// # Examples
/// ```
/// use clean_code_notes_exercises::exercises::exercise_01::{Point, Rectangle};
///
/// let origin = Point::new(0.0, 0.0);
/// let rectangle = Rectangle::new(origin, 2.0, 3.0);
///
/// assert_eq!(rectangle.width, 2.0);
/// assert_eq!(rectangle.height, 3.0);
/// assert_eq!(rectangle.get_area(), 6.0);
/// assert_eq!(rectangle.get_perimeter(), 10.0);
/// ```
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
    pub bounds: Bounds,
}

impl Rectangle {
    pub fn new(origin: Point, width: f64, height: f64) -> Rectangle {
        let bottom_left = origin.clone();
        let top_right = Point::new(origin.x + width, origin.y + height);
        let top_left = Point::new(origin.x, origin.y + height);
        let bottom_right = Point::new(origin.x + width, origin.y);

        let bounds = Bounds::new(top_left, top_right, bottom_right, bottom_left);

        Rectangle {
            width,
            height,
            bounds,
        }
    }

    pub fn get_area(&self) -> f64 {
        self.width * self.height
    }

    pub fn get_perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    pub fn print_coordinates(&self) {
        println!(
            "Top Left: ({}, {})",
            self.bounds.top_left.x, self.bounds.top_left.y
        );
        println!(
            "Top Right: ({}, {})",
            self.bounds.top_right.x, self.bounds.top_right.y
        );
        println!(
            "Bottom Right: ({}, {})",
            self.bounds.bottom_right.x, self.bounds.bottom_right.y
        );
        println!(
            "Bottom Left: ({}, {})",
            self.bounds.bottom_left.x, self.bounds.bottom_left.y
        );
    }
}

/// Builds a square with the given origin and side
///
/// # Examples
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_01::{Point, Rectangle, build_square};
///
/// let origin = Point::new(0.0, 0.0);
/// let square = build_square(origin, 2.0);
///
/// assert_eq!(square.width, 2.0);
/// assert_eq!(square.height, 2.0);
/// ```
pub fn build_square(origin: Point, side: f64) -> Rectangle {
    Rectangle::new(origin, side, side)
}
