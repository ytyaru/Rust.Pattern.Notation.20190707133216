/*
 * Rustパターン（記法）。
 * CreatedAt: 2019-07-07
 */
fn main() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("既存の値の変更を上書きできません");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("設定: {:?}", setting_value);
}
