enum SheetCells {
    Int(i32),
    Float(f64),
    Text(String),
}

fn printResult(row: Vec<SheetCells>, index: usize) {
    match &row[index] {
        &SheetCells::Int(i) => println!("this is integer = {:?}", i),
        &SheetCells::Float(i) => println!("this is float = {:?}", i),
        &SheetCells::Text(ref s) => println!("this is string = {:?}", s),
        _ => println!("this is not integer or float or string"),
    }
}

fn main() {
    // enum - match
    let row = vec![
        SheetCells::Int(5),
        SheetCells::Float(3.14),
        SheetCells::Text(String::from("Blue")),
    ];

    printResult(row, 2);
}
