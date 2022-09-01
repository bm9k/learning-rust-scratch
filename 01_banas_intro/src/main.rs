use std::io;

fn main() {
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
