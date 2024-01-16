#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {

    // read line from stdin
    let mut input = String::new();

    println!("Please input width and height of rectangle separated by space: ");
    std::io::stdin().read_line(&mut input).unwrap();

    // split input by space
    let mut iter = input.split_whitespace();

    // parse to u32
    let width = iter.next().unwrap().parse::<u32>().unwrap();
    let height = iter.next().unwrap().parse::<u32>().unwrap();

    let rect1 = Rectangle {
        width,
        height,
    };

    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
