// use std::io;

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope: {x}");
    }

    println!("The value of x: {x}");

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    println!("{x} {y}");

        // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("Sum: {sum}");
    println!("Difference: {difference}");
    println!("Product: {product}");
    println!("Quotient: {quotient}");
    println!("Truncated Quotient: {truncated}");
    println!("Remainder: {remainder}");

    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("t is {}", t);
    println!("f is {}", f);

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("Character c: {}", c);
    println!("Character z: {}", z);
    println!("Heart-eyes cat: {}", heart_eyed_cat);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("Five hundred: {}", five_hundred);
    println!("Six point four: {}", six_point_four);
    println!("One: {}", one);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5];

    println!("a[0]: {}", a[0]);
    println!("b[1]: {}", b[1]);

    // let a = [1, 2, 3, 4, 5];

    // println!("Please enter an array index.");

    // let mut index = String::new();

    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");

    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");

    // let element = a[index];

    // println!("The value of the element at index {index} is: {element}");

}
