fn main() {
    // Scalar:
    println!("======Scalar:======");
    // Integer Types
    println!("======Integer Types======");
    // let overflowing_u8:u8 = 256;
    let one_thousand = 1_000; // default is i32
    println!("The value of one_thousand is {}", one_thousand);
    // Floating-Point Types
    println!("======Floating-Point Types======");
    let float_x = 2.0; // default is f64
    println!("The value of float_x is {}", float_x);
    let float_y: f32 = 3.0;
    println!("The value of float_y is {}", float_y);

    // Numeric Operations
    println!("======Numeric Operations======");
    // addition
    let sum = 5 + 10;
    println!("The value of 5 + 10 is {}", sum);
    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of 95.5 - 4.3 is {}", difference);
    // multiplication
    let product = 4 * 30;
    println!("The value of 4 * 30 is {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("The value of 56.7 / 32.2 is {}", quotient);
    let floored = 2 / 3; // Results in 0
    println!("The value of 2 / 3 is {}", floored);

    // remainder
    let remainder = 43 % 5;
    println!("The value of 43 % 5 is {}", remainder);

    // The Boolean Type
    println!("======The Boolean Type======");
    let t = true;
    println!("The value of t is {}", t);
    let f: bool = 1 == 2; // with explicit type annotation
    println!("The value of f is {}", f);

    // The Character Type
    println!("======The Character Type======");
    let c = 'z'; // literals with single quotes
    println!("The value of c is {}", c);
    let z: char = 'ℤ'; // with explicit type annotation
    println!("The value of z is {}", z);
    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is {}", heart_eyed_cat);

    // Compound Types:
    println!("======Compound Types:======");
    // The Tuple Type
    println!("======The Tuple Type======");
    let tup = (500, 6.4, 1);

    let (tup_x, tup_y, tup_z) = tup;

    println!("The value of tup_x is: {}", tup_x);
    println!("The value of tup_y is: {}", tup_y);
    println!("The value of tup_z is: {}", tup_z);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);

    // The Array Type
    println!("======The Array Type======");
    let a = [1, 2, 3, 4, 5];
    println!("The value of a is {:?}", a);
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("The value of months is {:?}", months);
    for index in 0..months.len() {
        println!("The index: {}, value: {}", index, months[index]);
    }
    let b = [3; 5];
    println!("The value of b is {:?}", b);
}
