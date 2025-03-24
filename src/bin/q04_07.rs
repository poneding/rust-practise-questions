/// # Chapter 4 - Enum & Patterns
///
/// Use pattern matching to find that whether a point lies on X-Axis, Y-Axis or on which quadrant.
fn main() {
    let (x, y) = (3, 4);
    let p = find_point_location(x, y);
    assert_eq!(p, Point::Q1);

    let (x, y) = (-3, 4);
    let p = find_point_location(x, y);
    assert_eq!(p, Point::Q2);

    let (x, y) = (-3, -4);
    let p = find_point_location(x, y);
    assert_eq!(p, Point::Q3);

    let (x, y) = (3, -4);
    let p = find_point_location(x, y);
    assert_eq!(p, Point::Q4);

    let (x, y) = (0, 4);
    let p = find_point_location(x, y);
    assert_eq!(p, Point::OnY);

    let (x, y) = (3, 0);
    let p = find_point_location(x, y);
    assert_eq!(p, Point::OnX);
}

#[derive(Debug, PartialEq)]
enum Point {
    OnX, // x 轴上： y = 0
    OnY, // y 轴上： x = 0
    Q1,  // 第一象限： x > 0, y > 0
    Q2,  // 第二象限： x < 0, y > 0
    Q3,  // 第三象限： x < 0, y < 0
    Q4,  // 第四象限： x > 0, y < 0
}

fn find_point_location(x: i32, y: i32) -> Point {
    match (x, y) {
        (0, 0) => panic!("Origin is not a part of any quadrant"),
        (0, _) => Point::OnY,
        (_, 0) => Point::OnX,
        (x, y) if x > 0 && y > 0 => Point::Q1,
        (x, y) if x < 0 && y > 0 => Point::Q2,
        (x, y) if x < 0 && y < 0 => Point::Q3,
        (x, y) if x > 0 && y < 0 => Point::Q4,
        _ => panic!("Invalid point"),
    }
}
