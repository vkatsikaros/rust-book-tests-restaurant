#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }

    mod serving {
        fn take_order() {}
    }
}

pub fn eat() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
}