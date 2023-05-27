fn main() {
    // demo 1. lifetime annotations in funtion signatures
    let string1 = String::from("abcd");
    {
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }

    // demo 2. lifetime annotations in struct definitions
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("part: {}", i.announce_and_return_part("announcement"));

    // demo 3. static lifetime
    let s: &'static str = "I have a static lifetime.";
    println!("{s}");

    // demo 4. Generic Type Parameters, Trait Bounds, and Lifetimes Together
    {
        let string2 = "uvw";
        let result = longest_with_an_announcement(string1.as_str(), string2, "announcement");
        println!("The longest string is {}", result);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
