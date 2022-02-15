fn main() {}

// không có tham chiếu nên không cần sủ dụng lifetime
#[allow(dead_code)]
fn test1(param_1: Vec<f64>) -> Vec<f64> {
    param_1
}

// vì giá trị truyền vào có tham chiếu những giá trị return không có tham chiếu
// nên không sử dụng lifetime
// #[allow(dead_code)]
// fn test2<'a>(param_1: &'a Vec<f64>) -> Vec<f64> {
//     param_1
// }

// sử dụng lifetime vì có cả tham chiếu ở param truyền vào và return
#[allow(dead_code)]
fn test3(param_1: &Vec<f64>) -> &Vec<f64> {
    param_1
}
