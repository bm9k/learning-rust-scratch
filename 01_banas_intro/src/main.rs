use rand::Rng;
use std::{cmp::Ordering, io};

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

fn main() {
    // _basic_ui();
    // _numerics();
    //_float_precision_and_arithmetic();
    //_rand();
    //_conditional();
    // _arrays();
    _tuples();
}
