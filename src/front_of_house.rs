// the statement: mod front_of_house {}
// is no longer needed

pub mod hosting {
    pub fn add_to_waitlist() {}
}

mod serving {
    fn take_order() {}
}