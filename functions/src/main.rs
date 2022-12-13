fn main() {
    // Statements and Expressions

    // let y = 6; // Statements
    // let x = (let y = 6); // error

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    println!("Hello, world!");
    another_function();
    print_labeled_measurement(5, 'h');

    let result_of_sum = sum(100, 200);
    println!("The value of result_of_sum is {}", result_of_sum);
}

fn another_function() {
    println!("Another function.");
}

// Parameters
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

// Functions with Return Values
fn sum(a: i32, b: i32) -> i32 {
    a + b
}
