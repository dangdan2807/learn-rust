fn main() {
    let num1 = 10;
    let num2 = 20;
    let result_1 = get_ref(&num1);
    println!("result_1: {}", result_1);

    let result_2 = get_ref_2(&num1, num2);
    println!("result_2: {}", result_2);

    let result_3 = get_ref_3(&num1, &num2);
    println!("result_3: {}", result_3);

    let result_3_1 = get_ref_3_1(&num1, &num2);
    println!("result_3_1: {}", result_3_1);

    let result_4 = get_ref_4(&num1, &num2);
    println!("result_4: {}", result_4);

    let result_5 = get_ref_5(&num1, &num2);
    println!("result_5: {}", result_5);
}

// function này sẽ bị lỗi về life time vì biến a chỉ hoạt động trong hàm
// mà hàm này lại return giá trị tham chiếu biến a lúc này biến a trỏ đến 1 memory rỗng
// nên khi ra ngoài biến a sẽ trả về giá trị gì và lỗi
// fn get_ref() -> &i32 {
//     let a = 10;
//     &a
// }

// vì khi truyền vào là 1 tham chiếu nên khi return ra cũng là 1 tham chiếu nên hợp lệ
fn get_ref(param: &i32) -> &i32 {
    param
}

// chỉ thông báo là param_2 chưa được sử dụng
fn get_ref_2(param_1: &i32, param_2: i32) -> &i32 {
    return param_1;
}

// rust sẽ bảo lỗi life time vì rust sẽ không hiểu cả 2 param này được truyền từ đâu
// và life time tồn tại như thế nào
// fn get_ref_3(param_1: &i32, param_2: &i32) -> &i32 {
//     return param_1;
// }

// cách khắc phục là thêm life time
// fn get_ref_3<'a>(param_1: &'a i32, param_2: &'a i32) -> &'a i32 {
//     return param_1;
// }

// thêm 2 life time
fn get_ref_3<'a, 'b>(param_1: &'a i32, param_2: &'b i32) -> &'a i32 {
    println!("{}", param_2);
    // nếu ta đổi giá trị return thành param_2 thì sẽ bị lỗi vì không đúng life time
    // để khắc phục điều này ta đổi life time return từ 'a thành 'b
    // ví dụ như hàm get_ref_3_1
    // return param_2;
    return param_1;
}

fn get_ref_3_1<'a, 'b>(param_1: &'a i32, param_2: &'b i32) -> &'b i32 {
    println!("{}", param_2);
    // nếu ta đổi giá trị return thành param_2 thì sẽ bị lỗi vì không đúng life time
    // để khắc phục điều này ta đổi life time return từ 'a thành 'b
    // return param_2;
    return param_2;
}

// lỗi life time vì life time của param_1 và param_2 không cùng nhau
// để khắc phục điều này ta cho life time 'b là con của 'a
// fn get_ref_4<'a, 'b>(param_1: &'a i32, param_2: &'b i32) -> &'a i32 {
//     if param_1 > param_2 {
//         return param_1;
//     } else {
//         return param_2;
//     }
// }

fn get_ref_4<'a, 'b: 'a>(param_1: &'a i32, param_2: &'b i32) -> &'a i32 {
    if param_1 > param_2 {
        return param_1;
    } else {
        return param_2;
    }
}

// trên thực tế
// người ta thường dùng kiểu này và sẽ phân tích life time của 2 param có cùng 1 vị trí hay không 
// nếu cùng thì sẽ có cùng life time với nhau -> 2 param sẽ cùng tồn tại hoặc cùng chết (vòng đời)
// life time này không làm thay đổi vòng đời  của 2 param mà sẽ giúp 2 param này được tồn tại đến khi 
// cần sử dụng
fn get_ref_5<'a>(param_1: &'a i32, param_2: &'a i32) -> &'a i32 {
    if param_1 > param_2 {
        return param_1;
    } else {
        return param_2;
    }
}
