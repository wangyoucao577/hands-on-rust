// `#[allow(dead_code)]` is an attribute that disables the `dead_code` lint
#[allow(dead_code)]
fn fix_incorrect_order() {
    cook_order();

    // relative path with `super`
    super::deliver_order();
}

// `#[allow(dead_code)]` is an attribute that disables the `dead_code` lint
#[allow(dead_code)]
fn cook_order() {}

pub struct Breakfast {
    pub toast: String,

    // `#[allow(dead_code)]` is an attribute that disables the `dead_code` lint
    #[allow(dead_code)]
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

pub enum Appetizer {
    Soup,
    Salad,
}
