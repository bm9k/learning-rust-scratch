#![allow(unused)]

use rand::Rng;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::{cmp::Ordering, io};

mod restaurant;
use crate::restaurant::order_food;

fn _basic_ui() {
    println!("What is your name?");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Did not receive input");
    println!("Hello, {}, nice to meet you!", name.trim_end());

    const ONE_MIL: u32 = 1_000_000;
    // shadowing
    let age = "47";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
    age += 1;
    // subtle much?
    println!("I'm {} and I want ${}", age, ONE_MIL);
}

fn _numerics() {
    println!("Unsigned integers: u8, u16, u32, u64, u128, usize");
    println!("  - usize depends on computer");
    println!("Signed integers: i8, i16, i32, i64, i128, isize");
    println!("  - isize depends on computer");
    println!("Floats: f8, f16, f32, f64, f128 (no fsize)");

    println!();
    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max usize: {}", usize::MAX);
    println!();
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);
}

fn _float_precision_and_arithmetic() {
    // leading _ tells rust compiler to ignore unused variable errors (useful for placeholders)
    let _flag = true;
    let my_grade = 'c';

    // floating point precision
    let num_0 = "1.11111111111111111111";
    println!("lit: {}", num_0);
    let num_1: f32 = 1.11111111111111111111;
    println!("f32: {}", num_1);
    let num_2: f64 = 1.11111111111111111111;
    println!("f64: {}", num_2);

    // arithmetic
    let a: u32 = 5;
    let b: u32 = 4;
    println!("5 + 4 = {}", a + b);
    println!("5 - 4 = {}", a - b);
    println!("5 * 4 = {}", a * b);
    println!("5 / 4 = {}", a / b);
    println!("5 % 4 = {}", a % b);
}

fn _rand() {
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random number between 1 & 100: {}", random_num)
}

fn _conditional() {
    let age = 8;
    if (age >= 1) && (age <= 18) {
        println!("Important birthday");
    } else if (age == 21) || (age == 50) {
        println!("Other important birthday");
    } else if age >= 65 {
        println!("Another important birthday");
    } else {
        println!("Not important :(")
    }

    let done = true;
    // branch blocks can have multiple lines
    let label = if done { "Done" } else { "In-progress" };
    println!("{}", label);

    let age2: u32 = 8;
    match age2 {
        1..=18 => println!("Important birthday"),
        21 | 50 => println!("Other important birthday"),
        65..=u32::MAX => println!("Another important birthday"),
        _ => println!("Not important :("),
    }

    let my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't vote"),
        Ordering::Greater => println!("Can vote"),
        Ordering::Equal => println!("Recently gained the right to vote"),
    }
}

fn _arrays() {
    let arr_1 = [1, 2, 3, 4];
    println!("1st: {}", arr_1[0]);
    println!("Length: {}", arr_1.len());

    let arr_2 = [3, 1, 4, 1, 5, 9, 2, 6];
    let mut index = 0;

    println!("Infinite loop");
    loop {
        let is_odd = arr_2[index] % 2 == 1;
        println!(
            "{} is {}",
            arr_2[index],
            if is_odd { "odd" } else { "even" }
        );
        index += 1;
        if index == arr_2.len() {
            break;
        }
    }

    println!("While loop");
    index = 0;
    while index < arr_2.len() {
        let is_odd = arr_2[index] % 2 == 1;
        println!(
            "{} is {}",
            arr_2[index],
            if is_odd { "odd" } else { "even" }
        );
        index += 1;
    }

    println!("For loop");
    for value in arr_2.iter() {
        let is_odd = value % 2 == 1;
        println!("{} is {}", value, if is_odd { "odd" } else { "even" });
    }
}

fn _tuples() {
    // can be mutable, unlike python
    let mut my_tuple: (u8, String, bool) = (31, "Ben".to_string(), true);

    println!("Name: {}", my_tuple.1);

    my_tuple.2 = false;

    // tuple assignment/spread
    let (v1, v2, v3) = my_tuple;
    println!("v1: {}, v2: {}, v3: {}", v1, v2, v3);
}

fn _strings() {
    // two types of strings

    // String is mutable
    // &str is a pointer to string

    // variable length
    let mut st1 = String::new();
    st1.push('B');
    st1.push_str("en");
    st1.push_str(" test");
    for word in st1.split_whitespace() {
        println!("word: {}", word);
    }

    let st2 = st1.replace("e", "-3-");
    println!("{}", st2);

    // string of random characters
    let st3 = String::from("d p q q m b n a");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for ch in v1 {
        println!("char: {}", ch)
    }

    // dynamic heap string (String)
    // fixed length utf-8 byte sequence (str), which needs to be accessed through pointer (&str)
    // https://stackoverflow.com/a/24159933
    let st4: &str = "Random string";
    let mut st5: String = st4.to_string();
    let byte_arr1 = st5.as_bytes();
    // slice from 0 to 5 inclusive
    let st6 = &st5[0..6];
    println!("Length: {}", st6.len());
    st5.clear();
    println!("st5: {}", st5);
    let st7 = String::from("Just some");
    let st8 = String::from(" words in a string");
    // st7 does not exist anymore
    // st8 does exist, since it was referenced
    let st9 = st7 + &st8;

    for ch in st9.bytes() {
        println!("ch: {}", ch);
    }
}

fn _casting() {
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_32: u32 = (int_u8 as u32) + (int2_u8 as u32);

    println!("{} + {} = {}", int_u8, int2_u8, int3_32);
}

fn _enums() {
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    // define functions on enum
    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false,
            }
        }
    }

    let today = Day::Thursday;
    println!(
        "Today is a weekend? {}",
        if today.is_weekend() { "yes" } else { "no" }
    )
}

fn _returnless_fn(a: i32, b: i32) -> i32 {
    // no semi-colon
    a + b
}

fn _return_tuple(x: i32) -> (i32, i32) {
    return (x, x);
}

fn _vectors() {
    // can grow if mutable & can only contain values of the same type
    let vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = vec![1, 2, 3, 4];
    vec2.push(5);
    println!("vec2's first elem: {}", vec2[0]);
    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("2nd: {}", second),
        None => println!("No 2nd value"),
    }

    for i in &mut vec2 {
        *i *= 2;
    }

    for i in &vec2 {
        println!("{}", i);
    }
    println!("Vector length: {}", vec2.len());
    println!("Pop: {:?}", vec2.pop());
}

fn sum_i32(values: &[i32]) -> i32 {
    let mut total: i32 = 0;

    for value in values.iter() {
        total += value;
    }

    total
}

fn _fns() {
    println!("{}", _returnless_fn(4, 3));
    let (a, b) = _return_tuple(3);
    println!("{}, {}", a, b);

    let nums = vec![1, 2, 3, 4, 5];
    println!("Sum of nums: {}", sum_i32(&nums));
}

use std::ops::Add;

fn sum_ab<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

// TODO: figure this out
// fn sum<T:Add<Output = T>>(values: &[T]) -> T {
//     let mut total: T = 0;

//     for value: &T in values.iter() {
//         total += value;
//     }

//     total;
// }

fn print_str(x: String) {
    println!("A string: {}", x);
}

fn change_string(name: &mut String) {
    name.push_str(" is big");
}

fn _ownership() {
    // Heap: when putting data on the heap, you request a certain amount of space.
    // The OS finds space available and returns a pointer

    // Rules
    // 1. Each value has a variable that's called its owner
    // 2. There is only one owner at a time
    // 3. Whenever the owner goes out of scope, the value disappears (deleted & memory freed)

    let str1 = String::from("World");
    let str2 = str1;
    // str1 is moved to str2, so the following gives an error
    // println!("Hello {}", str1);
    let mut str3 = str2.clone();
    println!("hello {}", str2);
    println!("hello {}", str3);

    change_string(&mut str3);
    println!("Changed string: {}", str3);
}

fn _hash_maps() {
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("The Flash", "Barry Allen");

    for (k, v) in heroes.iter() {
        println!("{}'s real identity is {}", k, v);
    }

    println!("{} heroes", heroes.len());

    if heroes.contains_key("Iron Man") {
        println!("no info for Iron Man");
    }
}

fn _structs_and_traits() {
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }

    let mut bob = Customer {
        name: String::from("Bob Smith"),
        address: String::from("555 Main St"),
        balance: 420.69,
    };

    bob.address = String::from("554 Main St");

    // this example is a bit weird, but illustrates the point well
    // struct Rectangle<T, U> {
    //     length: T,
    //     height: U,
    // }
    // let rect = Rectangle {
    //     length: 4,
    //     height: 10.5,
    // };

    trait Shape {
        // constructor
        // fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    struct Rectangle {
        length: f32,
        width: f32,
    };

    struct Circle {
        radius: f32,
    };

    impl Shape for Rectangle {
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }

    impl Shape for Circle {
        fn area(&self) -> f32 {
            return self.radius * self.radius * PI;
        }
    }

    let c1 = Circle { radius: 5. };
    let r1 = Rectangle {
        length: 4.,
        width: 3.,
    };

    println!("area of c1: {}", c1.area());
    println!("area of r1: {}", r1.area());
}

fn _crates_modules_packages() {
    // Crates: modules that produce a library or executable
    // Modules: organise and handle privacy
    // Packages: build, test and share crates
    //      - packages can contain 0 or 1 library crates & unlimited binary crates
    // Paths: a way of naming an item such as a struct, function, etc.

    order_food();
}

fn _error_handling() {
    // panic!("Terrible error");
    let arr = [1, 2];
    // println!("element 10: {}", arr[10]);
}

fn _file_io_and_error_kinds() {
    let path = "lines.txt";

    let output = File::create(path);

    // Result has 2 variants: Ok & Err
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    // where T represents the data type of the value returned & E represents the type of error

    let mut output = match output {
        Ok(file) => file,
        Err(error) => panic!("Problem creating file: {:?}", error),
    };
    write!(output, "just some text\nseparated\nby\nnewlines").expect("Failed to write file");

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("line: {}", line.unwrap());
    }

    let output2 = File::create("random.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file {:?}", e),
            },
            _other_error => panic!("Problem opening file"),
        },
    };
}

fn _iterators() {
    let mut arr_it = [2, 7, 1, 8, 2, 8];

    // .iter borrows values from collection
    for value in arr_it.iter() {
        println!("value: {}", value);
    }

    // consumes collection
    // arr_it.into_iter()

    let mut iter1 = arr_it.iter();
    println!("1st: {:?}", iter1.next())
}

fn _closures() {
    // closure = anonymous function

    // let var_name = |parameters| -> return_type {
    //     BODY
    // }

    let can_vote = |age: u32| -> bool { age >= 18 };

    println!("Can 65 vote? {}", can_vote(65));
    println!("Can 15 vote? {}", can_vote(15));

    // closures can access variables outside their body
    let mut samp1 = 5;
    let print_var = || println!("samp1 = {}", samp1);
    print_var();

    samp1 = 10;
    let mut change_var = || samp1 += 1;
    change_var();
    println!("samp1 = {}", samp1);

    fn use_func<T>(a: i32, b: i32, func: T) -> i32
    where
        T: Fn(i32, i32) -> i32,
    {
        func(a, b)
    }

    let c = use_func(5, 3, |a, b| a + b);
    println!("result: {}", c);
}

fn _smart_pointers() {
    // smart pointers provide specific functionality beyond referencing a memory address

    // Box smart pointer stores data on the heap instead of the stack
    // data on stack must have a defined fixed size
    // Box normally used for large amount of data on heap with pointers to the data passed on the stack

    let b_int1 = Box::new(10);
    println!("b_int1 = {}", b_int1);

    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }

    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode {
                left: None,
                right: None,
                key,
            }
        }

        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }

        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }

    let a = TreeNode::new(5);
    let b = TreeNode::new(10);
    let c = TreeNode::new(15);
    let d = TreeNode::new(20);

    a.left(b).right(c);
    // why does this cause an error?
    // https://users.rust-lang.org/t/move-occurs-because-value-has-type-x-which-does-not-implement-the-copy-trait/40895
    // c.right(d);
}

fn _concurrency1() {
    // Common problems
    // 1. Threads access data in wrong order
    // 2. Threads are blocked from executing because of confusion over requirements to proceed

    let thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("spawned thread: {}", i);

            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..20 {
        println!("main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    thread1.join().unwrap();
}

fn _concurrency2() {
    // pub struct Bank {
    //     balance: f32,
    // }

    // fn withdraw(bank: &mut Bank, amount: f32) {
    //     bank.balance -= amount;
    // }

    // let mut bank = Bank{balance: 100.};

    // withdraw(&mut bank, 5.);
    // println!("Balance: {}", bank.balance);

    // withdraw(&mut bank, 2.5);
    // println!("Balance: {}", bank.balance);

    // fn customer(bank: &mut Bank) {
    //     withdraw(bank, 5.);
    // }

    // // closure cannot outlive current function
    // // smart pointers allow multiple owners & block access when needed
    // thread::spawn(|| {
    //     customer(&mut bank);
    // }).join().unwrap();

    pub struct Bank {
        balance: f32,
    }

    fn withdraw(bank: &Arc<Mutex<Bank>>, amount: f32) {
        let mut bank_ref = bank.lock().unwrap();
        if bank_ref.balance < amount {
            println!("Can't withdraw {} from balance of {}", amount, bank_ref.balance);
        } else {
            bank_ref.balance -= amount;
            println!("Withdrawal succeeded; {} remaining after {} withdrawn", bank_ref.balance, amount);
        }
    }

    fn customer(bank: &Arc<Mutex<Bank>>) {
        withdraw(&bank, 15.);
    }

    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank{balance: 100.}));

    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone();
        thread::spawn(move || {
            customer(&bank_ref);
        })
    });

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total: {}", bank.lock().unwrap().balance);


}

fn main() {
    // _basic_ui();
    // _numerics();
    //_float_precision_and_arithmetic();
    //_rand();
    //_conditional();
    // _arrays();
    // _tuples();
    // _strings();
    // _casting();
    // _enums();
    // _vectors();
    // _fns();
    // _ownership();
    // _hash_maps();
    // _structs_and_traits();
    // _crates_modules_packages();
    // _error_handling();
    // _file_io_and_error_kinds();
    // _iterators();
    // _closures();
    // _smart_pointers();
    // _concurrency1();
    _concurrency2();
}
