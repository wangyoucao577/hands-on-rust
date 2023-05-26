fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    notify_traits_as_parameters(&tweet);
    notify_traits_bound_syntax(&article);

    notify_traits_bound_syntax_both(&tweet, &article);

    let returned_summariable = returns_summarizable();
    println!("returned summary {}", returned_summariable.summarize());
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String, 
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    // leverage default implementation
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}

pub struct Tweet {
    pub username: String, 
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify_traits_as_parameters(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_traits_bound_syntax<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_traits_bound_syntax_both<T: Summary, U: Summary>(item1: &T, item2: &U) {
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
}

use std::fmt::Display;
use std::fmt::Debug;

#[allow(dead_code)]
pub fn notify_multiple_trait_impls(_item: &(impl Summary + Display)) {

}

#[allow(dead_code)]
pub fn notify_multiple_trait_bounds<T: Summary + Display>(_item: &T) {

}

#[allow(dead_code)]
fn some_function<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) -> i32 {
    1
}

#[allow(dead_code)]
fn some_function_with_where<T, U>(_t: &T, _u: &U) -> i32 
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
