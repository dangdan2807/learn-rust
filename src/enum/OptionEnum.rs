fn main() {
    let _x: i32 = 10;
    let _y: Option<i32> = Some(5);
    // cách cộng này sẽ gây ra lỗi
    // let _z = _x + _y;
    // số 0 là giá trị mặc định
    // nếu _y là số none thì sẽ cộng cho 0
    let mut _z: i32 = _x + _y.unwrap_or(0);
    println!("sum = {}", _z);
    let _y2: Option<i32> = None;
    _z = _x + _y2.unwrap_or(0);
    println!("sum = {}", _z);
}
