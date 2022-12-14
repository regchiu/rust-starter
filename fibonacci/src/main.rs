use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    println!(
        "The result of fibonacci_standar n = 5 is: {}",
        fibonacci_standard(5)
    );

    println!(
        "The result of fibonacci_recursive n = 5 is: {}",
        fibonacci_recursive(5)
    );

    let mut memo = HashMap::new();
    println!(
        "The result of fibonacci_memoization n = 5 is: {}",
        fibonacci_memoization(5, &mut memo)
    );

    println!("The result of FibIterator n = 5 is: {}", FibIterator::default().nth(5).unwrap());
}

fn fibonacci_standard(n: usize) -> usize {
    let mut a = 1;
    let mut b = 1;
    for _ in 1..n {
        let old = a;
        a = b;
        b += old;
    }

    b
}

fn fibonacci_recursive(n: usize) -> usize {
    match n {
        0 | 1 => 1,
        _ => fibonacci_recursive(n - 2) + fibonacci_recursive(n - 1),
    }
}

fn fibonacci_memoization(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if let Some(v) = memo.get(&n) {
        return *v;
    };

    let v = match n {
        0 | 1 => 1,
        _ => fibonacci_memoization(n - 2, memo) + fibonacci_memoization(n - 1, memo),
    };

    memo.insert(n, v);
    v
}

struct FibIterator {
    a: usize,
    b: usize,
}

impl Default for FibIterator {
    fn default() -> Self {
        FibIterator { a: 1, b: 1 }
    }
}

impl Iterator for FibIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.a;
        self.a = self.b;
        self.b = curr + self.a;

        Some(curr)
    }
}
