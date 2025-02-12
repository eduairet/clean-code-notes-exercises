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
/// let starting_point = Point::new(0.0, 0.0);
/// let rectangle = Rectangle::new(starting_point, 2.0, 3.0);
///
/// assert_eq!(rectangle.width, 2.0);
/// assert_eq!(rectangle.height, 3.0);
/// assert_eq!(rectangle.area(), 6.0);
/// assert_eq!(rectangle.perimeter(), 10.0);
/// ```
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
    pub bounds: Bounds,
}

impl Rectangle {
    pub fn new(starting_point: Point, width: f64, height: f64) -> Rectangle {
        let bottom_left = starting_point.clone();
        let top_right = Point::new(starting_point.x + width, starting_point.y + height);
        let top_left = Point::new(starting_point.x, starting_point.y + height);
        let bottom_right = Point::new(starting_point.x + width, starting_point.y);

        let bounds = Bounds::new(top_left, top_right, bottom_right, bottom_left);

        Rectangle {
            width,
            height,
            bounds,
        }
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    pub fn print(&self) {
        println!(
            "Starting Point: ({}, {})",
            self.bounds.bottom_left.x, self.bounds.bottom_left.y
        );
        println!(
            "Ending Point: ({}, {})",
            self.bounds.top_right.x, self.bounds.top_right.y
        );
    }
}
