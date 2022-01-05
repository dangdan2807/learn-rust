mod number;

fn main() {
    // biến khi được khởi tạo không thể cập nhật được nữa (tính năng cua rust)
    // cú pháp: let <tên biến>: <tên kiểu dữ liệu> = <giá trị>;
    let name: i32 = 32;
    // let str1: str = "hello world";
    let str2: String = "hello world".to_string();
    println!("kết quả: {}, {}", name, str2);

    // tạo biết có thể cập nhật giá trị
    let mut s: f32 = 3.14;
    println!("Trước khi cập nhật: {}", s);
    s = 3.0;
    println!("Sau khi cập nhật: {}", s);
    
    // có thể dùng _ để tách số
    let num: i128 = 100_000_000_000_000;
    println!("Kết quả: {}", num);


    // hằng
    static PI: f32 = 3.14;
    // const có phạm vi truy xuất nhỏ hơn so với hằng số static
    const PI1: f32 = 3.14;
    println!("kết quả: {} {}", PI, PI1);

    // ghi đè biến
    println!("trước khi ghi đè: {}", name);
    let name: String = "ghi".to_string();
    println!("Sau khi ghi đè: {}", name);
}
