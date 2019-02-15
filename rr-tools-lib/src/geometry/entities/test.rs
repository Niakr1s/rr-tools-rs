use super::*;

/// For tests with fig* at the end, see drawing from folder \test_files\explanation\
#[test]
fn eq_points_ref() {
    assert_eq!(
        &Point::new(1.0, 1.0, Some(1.0)),
        &Point::new(1.0, 1.0, Some(1.0))
    )
}
