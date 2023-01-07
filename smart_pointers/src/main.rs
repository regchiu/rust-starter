use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

// Enabling Recursive Types with Boxes
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

enum List2 {
    Cons(i32, Rc<List2>),
    Nil,
}

#[derive(Debug)]
enum List3 {
    Cons(Rc<RefCell<i32>>, Rc<List3>),
    Nil,
}

#[derive(Debug)]
enum List4 {
    Cons(i32, RefCell<Rc<List4>>),
    Nil,
}

impl List4 {
    fn tail(&self) -> Option<&RefCell<Rc<List4>>> {
        match self {
            List4::Cons(_, item) => Some(item),
            List4::Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list = {:?}", list);

    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let my_box = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, *my_box);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    // drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    let a = Rc::new(List2::Cons(
        5,
        Rc::new(List2::Cons(10, Rc::new(List2::Nil))),
    ));
    println!("count after creating a = {}", Rc::strong_count(&a));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    // The implementation of Rc::clone doesn’t make a deep copy of all the data like most types’ implementations of clone do.
    // The call to Rc::clone only increments the reference count, which doesn’t take much time.
    let b = List2::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = List2::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    let value = Rc::new(RefCell::new(5));

    let i = Rc::new(List3::Cons(Rc::clone(&value), Rc::new(List3::Nil)));
    let j = List3::Cons(Rc::new(RefCell::new(3)), Rc::clone(&i));
    let k = List3::Cons(Rc::new(RefCell::new(4)), Rc::clone(&i));

    *value.borrow_mut() += 10;

    println!("i after = {:?}", i);
    println!("j after = {:?}", j);
    println!("k after = {:?}", k);

    // Creating a Reference Cycle
    let l = Rc::new(List4::Cons(5, RefCell::new(Rc::new(List4::Nil))));

    println!("l initial rc count = {}", Rc::strong_count(&l));
    println!("l next item = {:?}", l.tail());

    let m = Rc::new(List4::Cons(10, RefCell::new(Rc::clone(&l))));

    println!("l rc count after m creation = {}", Rc::strong_count(&l));
    println!("m initial rc count = {}", Rc::strong_count(&m));
    println!("m next item = {:?}", m.tail());

    if let Some(link) = l.tail() {
        *link.borrow_mut() = Rc::clone(&m);
    }

    println!("m rc count after changing l = {}", Rc::strong_count(&m));
    println!("l rc count after changing m = {}", Rc::strong_count(&l));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("l next item = {:?}", l.tail());

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
// drop d than drop c
