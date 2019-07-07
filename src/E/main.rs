/*
 * Rustパターン（記法）。
 * CreatedAt: 2019-07-07
 */
fn main() {
    let s = Some(String::from("Hello!"));
//    if let Some(_s) = s { // error[E0382]: use of partially moved value: `s`
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);
}
