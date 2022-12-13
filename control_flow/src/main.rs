fn main() {
    // if Expressions
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    // Handling Multiple Conditions with else if
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let Statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // Returning Values from Loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // Loop Labels to Disambiguate Between Multiple Loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // Conditional Loops with while
    let mut number2 = 3;

    while number2 != 0 {
        println!("{number2}!");

        number2 -= 1;
    }

    println!("LIFTOFF!!!");

    // Looping Through a Collection with white/for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    // while
    println!("======while======");
    // this approach is error prone
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    // For
    println!("======for======");
    for element in a {
        println!("the value is: {element}");
    }
}
