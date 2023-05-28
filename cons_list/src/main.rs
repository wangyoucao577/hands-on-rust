use std::rc::Rc;

fn main() {
    // let b = Box::new(5);
    // println!("Hello, world! b = {b}");

    // cons list by Box
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    println!("{:?}", list);
    println!("");

    // cons list by Rc
    let a = Rc::new(ListByRc::Cons(
        5,
        Rc::new(ListByRc::Cons(10, Rc::new(ListByRc::Nil))),
    ));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = ListByRc::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = ListByRc::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
enum ListByRc {
    Cons(i32, Rc<ListByRc>),
    Nil,
}
