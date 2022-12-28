use std::collections::HashMap;

fn main() {
    // Vector
    let mut v1: Vec<i32> = Vec::new();
    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);
    println!("v1: {:?}", v1);

    let v2 = vec![1, 2, 3];
    println!("v2: {:?}", v2);

    // Reading Elements of Vectors
    let v3 = vec![1, 2, 3, 4, 5, 6];
    let third: &i32 = &v3[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v3.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    // let does_not_exist1 = &v3[100]; // will cause the program to panic because it references a nonexistent element.
    let does_not_exist2 = v3.get(100);
    println!("{:?}", does_not_exist2);

    // Iterating over the Values in a Vector
    let v4 = vec![100, 32, 57];
    for i in &v4 {
        println!("{i}");
    }

    let mut v5 = vec![100, 32, 57];

    for i in &mut v5 {
        *i += 50;
        println!("{i}");
    }

    // Using an Enum to Store Multiple Types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);

    {
        let v6 = vec![1, 2, 3, 4];

        // do stuff with v6
    } // <- v6 goes out of scope and is freed here

    // String::from same as to_string()
    let data = "initial contents";
    let s1 = data.to_string();
    let s2 = String::from(data);

    // Updating a String
    let mut s3 = String::from("foo");
    let s4 = "bar";
    s3.push_str(s4);
    println!("s3 is {}", s3);

    // Concatenation with the + Operator or the format!
    let s5 = String::from("Hello, ");
    let s6 = String::from("world!");
    let s7 = s5 + &s6; // note s5 has been moved here and can no longer be used

    // The + operator uses the add method, whose signature looks something like this:
    // fn add(self, s: &str) -> String {}

    let s8 = String::from("tic");
    let s9 = String::from("tac");
    let s10 = String::from("toe");
    // let s11 = s8 + "-" + &s9 + "-" + &s10;
    let s11 = format!("{}-{}-{}", s8, s9, s10);
    println!("{}", s8);
    println!("s11: {}", s11);

    let hello = "Здравствуйте";
    // let h = &hello[0..1]; // will cause the program to panic

    // Methods for Iterating Over Strings
    for c in hello.chars() {
        println!("{}", c);
    }

    for c in hello.bytes() {
        println!("{}", c);
    }

    // Storing Keys with Associated Values in Hash Maps
    let mut scores = HashMap::new();
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    scores.insert(blue, 10);
    scores.insert(yellow, 8);
    println!("scores: {:?}", scores);
    // borrow of moved value: `blue` and `yellow` value borrowed here after move
    // println!("blue: {}, yellow: {}", blue, yellow);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 9];
    let scores2: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("scores2: {:?}", scores2);

    // Accessing Values in a Hash Map
    let team_name = String::from("Blue");
    let blue_score = scores2.get(&team_name);
    println!("blue_score: {:?}", blue_score);

    for (key, value) in &scores2 {
        println!("{key}: {value}");
    }

    // Updating a Hash Map

    // Overwriting a Value
    let mut scores3 = HashMap::new();

    scores3.insert(String::from("Blue"), 10);
    scores3.insert(String::from("Blue"), 25);

    println!("{:?}", scores3);

    // Adding a Key and Value Only If a Key Isn’t Present
    let mut scores4 = HashMap::new();
    scores4.insert(String::from("Blue"), 10);

    scores4.entry(String::from("Yellow")).or_insert(50);
    scores4.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores4);

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
