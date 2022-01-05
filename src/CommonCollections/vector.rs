fn main() {
    // array
    let array_a = [1, 2, 3, 4, 5];
    println!("array: {:?}", array_a);
    // vector
    let mut vec_1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("vec_1: {:?}", vec_1);

    let mut vec_2 = Vec::new();
    vec_2.push(1);
    vec_2.push(2);
    vec_2.push(3);
    println!("vec_2: {:?}", vec_2);

    // get index
    let vec_value = &vec_1[2];
    println!("vec_1[2]: {:?}", vec_value);

    let index = 8;
    match vec_1.get(index) {
        Some(vec_value) => println!("Some(vec_value): this is {:?} element = {:?}", index, vec_value),
        vec_value => println!("(vec_value): this is {:?} element = {:?}", index, vec_value),
        None => println!("this is None value"),
    }

    // for
    for i in &mut vec_1 {
        *i += 10;
        println!("{}", i);
    }
}
