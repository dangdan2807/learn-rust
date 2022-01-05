fn main() {
    let _s1 = String::from("hello");
    // đúng
    let _s2 = _s1.clone();
    // sai
    // let _s2 = _s1;
    // vì khi 2 biến trỏ đến cùng 1 bộ nhớ thì biến trỏ đến sau sẽ được sử dụng bộ nhớ đó và biến thứ nhất sẽ bị drop khỏi bộ nhớ
    // để khắc phục điều này thì ta sẽ copy ra giá trị đó ra vùng nhớ mới bằng hàm clone()
    println!("{}", _s1);
    println!("{}", _s2);
    // Ownership
    let _a1 = gives_ownership();
    let _a2 = String::from("hello _a2");
    let _a3 = takes_and_gives_back(_a2);
    // nến in ra biến _a2 sẽ bị lỗi vì lúc này địa chỉ (Ownership) của biến đã được trỏ qua biến _a3 khi sử dụng hàm takes_and_gives_back()
    println!("{}, {}", _a1, _a3);
}

fn gives_ownership() -> String {
    return String::from("hello _a1");
}

fn takes_and_gives_back(some_string: String) -> String {
    return some_string;
}
