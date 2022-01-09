// nếu struct chỉ có mỗi T thì cả x, y khi được tạo thì phải chung kiểu dữ liệu
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixed<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = get_large_number(number_list);
    println!("The largest number is {}", largest);
    let number_list_2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let largest_2 = get_large(number_list_2);
    println!("The largest 2 is {}", largest_2);

    let number_list_3 = vec!['a', 'b', 'j', 'x', 'e', 'f'];
    let largest_3 = get_large(number_list_3);
    println!("The largest 3 is {}", largest_3);

    let p1 = Point { x: 0, y: 7 };
    println!("P1: {:?}", p1);
    let p2 = Point { x: 4.3, y: 'c' };
    println!("P2: {:?}", p2);
    let p3 = p1.mixed(p2);
    println!("P3: {:?}", p3);
}

// generic types là một kiểu dữ liệu chung cho các kiểu dữ liệu khác nhau
// kí hiệu: T
fn get_large<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn get_large_number(list: Vec<i32>) -> i32 {
    let mut largest = list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
