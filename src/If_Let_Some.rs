fn main() {
    let _value = Some(4);
    if let Some(5) = _value {
        println!("bang 5")
    } else if let Some(4) = _value {
        println!("bang 4")
    }
}
