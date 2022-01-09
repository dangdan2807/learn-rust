struct Data {
    num1: i32,
    num2: i32,
    str1: String,
    optional: Option<i32>,
}

struct Data2 {
    num1: i32,
    num2: i32,
    str1: String,
    optional: Option<i32>,
}

impl Data {
    fn new() -> Data {
        Data {
            num1: 15,
            num2: 25,
            str1: "some string 2".to_string(),
            optional: None,
        }
    }
}

// trait hỗ trợ tạo các hàm mặc định(hàm đã được code chức năng bên trong) cho struct
trait Transform {
    // hàm mặc định
    fn revert(&self) -> String {
        String::from("no string....")
    }

    fn output_revert(&self) {
        println!("{}", self.revert());
    }
}

impl Transform for Data {
    fn revert(&self) -> String {
        self.str1.chars().rev().collect::<String>()
    }
}

impl Transform for Data2 {
    fn revert(&self) -> String {
        (self.num1 + self.num2).to_string()
    }
}

fn main() {
    let data1 = Data::new();
    data1.output_revert();

    let data2 = Data2 {
        num1: 10,
        num2: 20,
        str1: String::from("ok string"),
        optional: None,
    };
    println!("{}", data2.revert());
}
