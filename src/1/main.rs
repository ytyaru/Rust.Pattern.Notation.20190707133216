/*
 * Rustパターン（記法）。
 * CreatedAt: 2019-07-07
 */

fn main() {
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
//        Some(5) => println!("Got 5"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);
}
