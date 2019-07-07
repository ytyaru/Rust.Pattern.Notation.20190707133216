/*
 * Rustパターン（記法）。
 * CreatedAt: 2019-07-07
 */
fn main() {
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
//        (first, ..) => println!("{}", first), // 2
//        (.., last) => println!("{}", last), // 32
//        (.., second, ..) => println!("{}", second), // error: `..` can only be used once per tuple or tuple struct pattern
        (first, .., last) => {
            println!("{}, {}", first, last); // 2, 32
        },
    }
}
