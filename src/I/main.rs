/*
 * Rustパターン（記法）。
 * CreatedAt: 2019-07-07
 */
fn main() {
    let robot_name = Some(String::from("Bors"));
    match robot_name {
//        Some(name) => println!("Found a name: {}", name),
        Some(ref name) => println!("Found a name: {}", name),
        None => (),
    }
    println!("robot_name is: {:?}", robot_name);
}
