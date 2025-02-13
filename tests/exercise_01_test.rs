use clean_code_notes_exercises::exercises::exercise_01::*;

#[cfg(test)]
mod exercise_01_test {
    use super::*;

    #[test]
    fn test_exercise_01_rectangle() {
        let rectangle = Rectangle::new(Point::new(0.0, 0.0), 2.0, 3.0);
        rectangle.print_coordinates();
        assert_eq!(rectangle.get_area(), 6.0);
        assert_eq!(rectangle.get_perimeter(), 10.0);
        assert_eq!(rectangle.bounds.bottom_left.x, 0.0);
        assert_eq!(rectangle.bounds.bottom_left.y, 0.0);
        assert_eq!(rectangle.bounds.bottom_right.x, 2.0);
        assert_eq!(rectangle.bounds.bottom_right.y, 0.0);
        assert_eq!(rectangle.bounds.top_left.x, 0.0);
        assert_eq!(rectangle.bounds.top_left.y, 3.0);
        assert_eq!(rectangle.bounds.top_right.x, 2.0);
        assert_eq!(rectangle.bounds.top_right.y, 3.0);
    }

    #[test]
    fn test_exercise_01_square() {
        let square = build_square(Point::new(0.0, 0.0), 3.0);
        square.print_coordinates();
        assert_eq!(square.get_area(), 9.0);
        assert_eq!(square.get_perimeter(), 12.0);
        assert_eq!(square.bounds.bottom_left.x, 0.0);
        assert_eq!(square.bounds.bottom_left.y, 0.0);
        assert_eq!(square.bounds.bottom_right.x, 3.0);
        assert_eq!(square.bounds.bottom_right.y, 0.0);
        assert_eq!(square.bounds.top_left.x, 0.0);
        assert_eq!(square.bounds.top_left.y, 3.0);
        assert_eq!(square.bounds.top_right.x, 3.0);
        assert_eq!(square.bounds.top_right.y, 3.0);
    }
}
