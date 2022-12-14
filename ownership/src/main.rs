fn main() {
    // Variable Scope
    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward
                         // do stuff with s
        println!("s = {}", s);
    } // this scope is now over, and s is no longer valid

    // The String Type
    {
        let mut s = String::from("hello"); // s is valid from this point forward
                                           // do stuff with s
        s.push_str(", world!");
        println!("s = {}", s);
    } // this scope is now over, and s is no longer valid

    // Move
    {
        // In this example, we would say that s1 was moved into s2.
        let s1 = String::from("hello");
        let s2 = s1;
        // With only s2 valid, when it goes out of scope, it alone will free the memory, and we’re done.
        // println!("s1 = {}", s1); // value borrowed here after move
        println!("s2 = {}", s2);
    }

    // Clone
    // When you see a call to clone, you know that some arbitrary code is being executed and that code may be expensive.
    // It’s a visual indicator that something different is going on.
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1 = {}, s2 = {}", s1, s2);
    }

    // Copy
    {
        let x = 5;
        let y = x;
        println!("x = {}, y = {}", x, y);
    }

    // Ownership and Functions
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward

    // Return Values and Scope
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1
    println!("{}", s1);

    let s2 = String::from("hello"); // s2 comes into scope
    println!("{}", s2);

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
    println!("{}", s3);

    // Reference
    let s4 = String::from("hello");

    let len = calculate_length(&s4);

    println!("The length of '{}' is {}.", s4, len);

    // Mutable References
    let mut s5 = String::from("hello");
    change(&mut s5);

    // Dangling References
    // let reference_to_nothing = dangle();

    // The Slice Type

    let s6 = String::from("hello world");

    let word = first_word(&s6); // word will get the value 5

    // s6.clear(); // this empties the String, making it equal to ""
    println!("s6 = {}", s6);
    println!("word = {}", word);
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    // Other Slices
    let a = [1, 2, 3, 4, 5, 6];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}
// Here, x goes out of scope, then s. But because s's value was moved, nothing
// special happens.

// Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
// happens. s1 goes out of scope and is dropped.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn change(s: &mut String) {
    s.push_str(", world!"); // `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
}

// fn dangle() -> &String {
//     // dangle returns a reference to a String
//     let s = String::from("hello"); // s is a new String
//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
