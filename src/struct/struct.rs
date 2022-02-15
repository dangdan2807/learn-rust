#[derive(Debug)]
struct Member {
    name: String,
    email: String,
    age: u64,
    active: bool,
}

fn create_member(name: String, email: String, age: u64) -> Member {
    return Member {
        name: name,
        email: email,
        age,
        active: true,
    };
}

fn main() {
    // thêm mut để có thể thay đổi dư liệu trong biến
    let mut member1: Member = create_member(
        String::from("nguyen van a"),
        String::from("nguyenvana@gmail.com"),
        28,
    );
    println!("old member1: {:#?}", member1);

    member1.name = String::from("nguyen van b");
    println!("new member1: {:#?}", member1);
    
    let member2: Member = Member {
        name: String::from("nguyen van member3"),
        ..member1
    };
    println!("new member2: {:?}", member2);
}
