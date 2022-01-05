fn main() {
    // String
    // UTF-8

    // cách tạo String
    let s1 = String::from("a String 1");
    let s2 = String::new();
    let s3: &str = "a String 3";
    let s4 = s3.to_string();

    let s5 = String::from("xin chào");
    // tiếng nga
    let s6 = String::from("Привет");
    // ký tự đặc biệt - icon
    let s7 = String::from("🌴🐻🌷🤖😨🤑");
    println!("{:?}", s1);
    println!("{:?}", s2);
    println!("{:?}", s3);
    println!("{:?}", s4);
    println!("{:?}", s5);
    println!("{:?}", s6);
    println!("{:?}", s7);

    // nối chuỗi
    // không nên sử dụng + s3 vì sẽ gây ra lỗi
    // vì vậy hãy tham chiếu đến biến s6
    // lúc này quyền sở hữu chuỗi của s5 sẽ được chuyển sang cho s8
    // còn s6 vẫn dùng bình thường vì chỉ tham chiếu
    let s8 = s5 + " in Russia is " + &s6;

    println!("{:?}", s8);
}