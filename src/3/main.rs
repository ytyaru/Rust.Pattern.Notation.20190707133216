/*
 * Rustパターン（記法）。
 * CreatedAt: 2019-07-07
 */
fn main() {
    let x = 5;
    match x {
        1 ... 5 => println!("one through five"),
        _ => println!("something else"),
    }
}
