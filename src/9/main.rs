/*
 * Rustパターン（記法）。
 * CreatedAt: 2019-07-07
 */
struct Point {
    x: i32,
    y: i32,
}
fn main() {
    let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
}
