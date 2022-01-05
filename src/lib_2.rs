mod front_house;
// use rand::Rng;

mod back_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        pub fruit: String,
    }

    #[derive(Debug)]
    pub enum Salad {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn monday(toast: &str) -> Breakfast {
            return Breakfast {
                toast: String::from(toast),
                fruit: String::from("banana"),
            };
        }
    }
}

fn eat_res() {
    front_house::hosting::add_to_wait_list();
}

fn main() {
    let mut order1 = back_house::Breakfast::monday("Fish");
    println!("{:?}", order1);
    
    // cách thuộc tính trong struct phải public mới có thể update
    order1.toast = String::from("Chicken");
    println!("{:?}", order1);
    let order2 = back_house::Breakfast {
        toast: String::from("beef"),
        fruit: String::from("apple"),
    };
    println!("{:?}", order2);
    let order3 = back_house::Salad::Salad;
    println!("{:?}", order3);
    let order4 = back_house::Salad::Soup;
    println!("{:?}", order4);

    eat_res();
}
