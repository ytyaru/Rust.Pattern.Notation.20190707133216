/*
 * Rustパターン（記法）。
 * CreatedAt: 2019-07-07
 */
fn main() {
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        },
    }
}
