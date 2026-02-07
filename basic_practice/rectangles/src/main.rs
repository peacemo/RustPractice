#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32, 
}

impl Rectangle {  // impl methods with self as parameter, call by `.`
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

impl Rectangle {  // impl methods without self as parameter, call by `::`
    fn square(size: u32) -> Self {
        Rectangle {
            width: size, 
            height: size,
        }
    }
}

fn main() {
    let width = 30;
    let height = 50;
    let rectangle = Rectangle {
        width,
        height,
    };

    let rectangle2 = Rectangle {
        width: rectangle.width, 
        height: 60
    };

    let rectangle3 = Rectangle::square(30);

    println!(
        "The area of the rectangle1 is {} square pixels. ", 
        rectangle.area(),
    );

    println!(
        "The area of the rectangle2 is {} square pixels. ", 
        rectangle2.area(),
    );

    println!(
        "The area of the rectangle3 is {} square pixels. ", 
        rectangle3.area(),
    );

    println!("Rect1 is: {rectangle:#?}");

    println!("Rect2 is: {rectangle2:?}");

    println!("Rect3 is: {rectangle3:?}");

    println!("Can rectandgle hold rectangle2? {}", rectangle.can_hold(&rectangle2));

    println!("Can rectangle2 hold rectangle1? {}", rectangle.can_hold(&rectangle)); 

    println!("Can rectangle hold rectangle3? {}", rectangle.can_hold(&rectangle3));
}
