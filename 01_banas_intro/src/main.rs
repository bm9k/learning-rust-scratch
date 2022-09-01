use std::io;

fn _basic_ui() {
    println!("What is your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Did not receive input");
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

fn main() {
    // _basic_ui();
    // _numerics();
    _float_precision_and_arithmetic();
}
