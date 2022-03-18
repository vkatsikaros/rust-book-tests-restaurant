#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }

    mod serving {
        fn take_order() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast:&str) -> Breakfast {
            Breakfast{
                toast: String::from(toast),
                fruit: String::from("peach"),
            } 
        }
    }
}

pub fn eat() {
    // create summer breakfast with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // change meal's toast
    meal.toast = String::from("Wheat");

    // this is not allowed, the field fruit is not pub
    // meal.fruit = String::from("Stawberry");
    println!("a {} toast please!", meal.toast);
}