#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
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

// Using a semicolon after mod front_of_house rather than using a
// block tells Rust to load the contents of the module from another
// file with the same name as the module
mod front_of_house;
use self::front_of_house::hosting;

pub fn eat() {

    hosting::add_to_waitlist();

    // create summer breakfast with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // change meal's toast
    meal.toast = String::from("Wheat");

    // this is not allowed, the field fruit is not pub
    // meal.fruit = String::from("Stawberry");
    println!("a {} toast please!", meal.toast);
}