/*
 * Rustパターン（記法）。
 * CreatedAt: 2019-07-07
 */
struct Point {
    x: i32,
    y: i32,
    z: i32,
}
fn main() {
    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
}
