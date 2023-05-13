fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    dbg!(&rect1);
    println!("rectangle: {:#?}", &rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rect: &Rectangle) -> u32 {
    return rect.width * rect.height;
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
