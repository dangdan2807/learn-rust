fn main() {
    // khi tham chiếu đối với biến có thể thay đổi (có mut)
    // thì không thêm tham chiếu bằng 2 biến cùng 1 - gây ra lỗi
    let mut _s1 = String::from("hello");
    let _length = calculate_length(&_s1);
    // bình thường
    println!("do dai cua '{}' la {}", _s1, _s1.len());
    // Reference
    println!("do dai cua '{}' la {}", _s1, _length.to_string());
    let _length2 = calculate_length_and_push(&mut _s1);
    println!("do dai cua '{}' la {}", _s1, _length2.to_string());
}

// hàm này chỉ cho tham chiếu và không được sửa đổi
fn calculate_length(some_string: &String) -> usize {
    let _length = some_string.len();
    return _length;
}

// hàm này chỉ cho tham chiếu và được sửa đổi
fn calculate_length_and_push(some_string: &mut String) -> usize {
    some_string.push_str(" world");
    let _length = some_string.len();
    return _length;
}
