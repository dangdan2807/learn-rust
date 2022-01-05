fn main() {
    // tuple
    let tup = ("hello", 100_000);
    println!("{:?}", tup);
    println!("{}", tup.0);
    println!("{}", tup.1);

    let (_string, _int) = tup;
    println!("{}", _string);
    println!("{}", _int);

    // array
    let arr: [i32; 3] = [100, 200, 300];
    let arr2 = [0; 3];
    println!("{:?}", arr);
    println!("{}", arr[0]);

    for i in arr2.iter() {
        println!("{} ", i);
    }
}
