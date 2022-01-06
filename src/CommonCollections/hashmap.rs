use std::collections::HashMap;

fn main() {
    // hashmap
    let mu = String::from("mu");
    let mc = String::from("mc");

    let mut scores = HashMap::new();

    scores.insert(mu, 10);
    scores.insert(mc, 9);

    for (key, value) in &scores {
        println!("{} {}", key, value);
    }

    let team_name = String::from("mu");
    // scores.get() trả về 1 biến kiểu Option<&V>
    let score = scores.get(&team_name);
    println!("score = {:?}", score);

    // c2
    let mut scores2 = HashMap::new();
    scores2.insert(String::from("mu"), 10);
    // nếu trùng key sẽ lấy giá trị mới nhất
    scores2.insert(String::from("mu"), 20);

    // c3: nếu giá trị không tồn tại thì sẽ thêm vào giá trị đầu tiên được chèn vào
    // nếu giá trị đã tồn tại thì sẽ bỏ qua
    scores2.entry(String::from("mc")).or_insert(10);
    scores2.entry(String::from("mc")).or_insert(20);

    println!("scores2 = {:?}", scores2);

    // ứng dụng
    let text = "hello world this is wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
